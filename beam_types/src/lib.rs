#[derive(Debug, serde::Serialize,serde::Deserialize)]
pub struct BeamState {
    pub slot: u64,
}