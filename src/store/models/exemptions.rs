pub struct Exemptions {
    _media_exemptions: Vec<usize>,
    _user_exemptions: Vec<usize>,
}

impl Exemptions {
    /// Converts a vector of `usize` values to a vector of `u8` values using JSON serialization.
    ///
    /// # Arguments:
    ///
    /// * `exemptions`: Reference to a vector of `usize` that is serialized into a JSON byte vector using
    /// `serde_json::to_vec` and returns the resulting byte vector.
    ///
    /// # Returns:
    ///
    /// Serialized JSON bytes of the input `exemptions` vector.
    pub fn to_vec_from_exemptions(exemptions: &Vec<usize>) -> Vec<u8> {
        let exemption_bytes =
            serde_json::to_vec(exemptions).expect("Failed to serialize exemptions to JSON");
        exemption_bytes
    }

    /// Deserializes a byte vector into a vector of `usize` values using serde_json.
    ///
    /// # Arguments:
    ///
    /// * `bytes`: Option<Vec<u8> from the jammdb
    ///
    /// # Returns:
    ///
    /// A `Vec<usize>` representing the overseer request IDs
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
