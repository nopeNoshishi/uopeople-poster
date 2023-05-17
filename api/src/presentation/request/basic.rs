use serde::Deserialize;

#[derive(Deserialize)]
pub struct PrimeRequest {
    pub base: usize,
}
