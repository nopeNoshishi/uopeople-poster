use serde::Serialize;

#[derive(Serialize)]
pub struct PrimeSerachedResponse {
    pub primes: Vec<usize>,
}
