pub struct Exemptions {
    _media_exemptions: Vec<usize>,
    _user_exemptions: Vec<usize>,
}

impl Exemptions {
    pub fn to_vec_from_exemptions(exemptions: &Vec<usize>) -> Vec<u8> {
        let exemption_bytes =
            serde_json::to_vec(exemptions).expect("Failed to serialize exemptions to JSON");
        exemption_bytes
    }

    pub fn to_exemptions_from_vec(bytes: Option<Vec<u8>>) -> Vec<usize> {
        match bytes {
            Some(bytes) => {
                let exemptions: Vec<usize> =
                    serde_json::from_slice(&bytes).expect("Failed to deserialize exemptions");
                exemptions
            }
            None => Vec::<usize>::new(),
        }
    }
}
