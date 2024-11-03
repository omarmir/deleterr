use std::{sync::Arc, time::Duration};

use actix_web::rt::time::interval;
use actix_web_lab::{
    sse::{self, Sse},
    util::InfallibleStream,
};
use futures_util::future;
use parking_lot::Mutex;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<(Uuid, SSEType, mpsc::Sender<sse::Event>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SSEType {
    Requests,
    SeriesDeletion,
}

pub enum MessageType {
    Progress((usize, usize)),
    Waiting,
    Completion,
    Error(String),
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });

        Broadcaster::spawn_ping(Arc::clone(&this));

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
    /// list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();

        let mut ok_clients = Vec::new();

        for client in clients {
            if client
                .2
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(
        &self,
        uuid: Uuid,
        sse_type: SSEType,
    ) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
        let (tx, rx) = mpsc::channel(10);

        tx.send(sse::Event::Comment("connected".into()))
            .await
            .unwrap();

        self.inner.lock().clients.push((uuid, sse_type, tx));

        Sse::from_infallible_receiver(rx)
    }

    pub fn close_client(&self, client_id: Uuid) {
        self.inner
            .lock()
            .clients
            .retain(|(id, _, _)| *id != client_id);
    }

    pub fn close_clients_of_type(&self, remove_type: SSEType) {
        self.inner
            .lock()
            .clients
            .retain(|(_, sse_type, _)| *sse_type != remove_type);
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, message_type: MessageType) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients.iter().map(|client| match &message_type {
            MessageType::Progress(prog) => client.2.send(
                sse::Data::new(
                    serde_json::json!({"progress": prog.0, "total": prog.1})
                        .to_string()
                        .as_str(),
                )
                .event("progress")
                .into(),
            ),
            MessageType::Error(err) => client
                .2
                .send(sse::Data::new(err.as_str()).event("error").into()),
            MessageType::Waiting => client
                .2
                .send(sse::Data::new("waiting").event("waiting").into()),
            MessageType::Completion => client
                .2
                .send(sse::Data::new("complete").event("completion").into()),
        });

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }
}
