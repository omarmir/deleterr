use persy::{Config, Persy, PersyId, ValueMode};
use std::{fmt, sync::Mutex};

pub struct PersyManager {
    pub persy: Mutex<Persy>,
}

impl fmt::Debug for PersyManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement your custom formatting logic here
        write!(
            f,
            "PersyManager {{ persy.is_poisoned: {:?} }}",
            self.persy.is_poisoned()
        )
    }
}

impl PersyManager {
    pub fn new() -> Self {
        PersyManager {
            persy: Mutex::new(
                Persy::open_or_create_with("deleterr2.persy", Config::default(), |persy| {
                    // this closure is only called on database creation
                    let mut tx = persy.begin()?;
                    tx.create_segment("services")?;
                    tx.create_segment("media_exemptions")?;
                    tx.create_segment("user_exemption")?;
                    tx.create_index::<String, PersyId>("services_index", ValueMode::Cluster)?;
                    tx.create_index::<String, PersyId>(
                        "media_exemption_index",
                        ValueMode::Cluster,
                    )?;
                    tx.create_index::<String, PersyId>("user_exemption_index", ValueMode::Cluster)?;
                    let prepared = tx.prepare()?;
                    prepared.commit()?;
                    println!("Segment and Index successfully created");
                    Ok(())
                })
                .expect("Unable to create persy store!"),
            ),
        }
    }
}
