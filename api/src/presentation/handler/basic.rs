use super::super::request::basic::*;
use super::super::response::basic::*;

use axum::{extract::Query, response::IntoResponse, Json};
use tracing::log::info;

pub async fn search(prime: Query<PrimeRequest>) -> impl IntoResponse {
    info!("Target Number: {:?}", prime.base);

    let primes: Vec<usize> = prime_serch(prime.base);

    info!("Result {:?}", primes);

    Json(PrimeSerachedResponse { primes })
}

// TODO: Library crateから引っ張ってくる
fn prime_serch(number: usize) -> Vec<usize> {
    // Base candidate number for prime
    let mut candidate = Vec::<usize>::new();
    for i in 2..(number + 1) {
        candidate.push(i);
    }

    // Prime vector
    let mut primes = Vec::<usize>::new();

    // Limit
    let lim = (number as f64).sqrt().floor() as usize;

    // Start
    let mut prime = 2_usize;
    loop {
        // Add
        primes.push(prime);

        // Sieve
        sieve(prime, &mut candidate);

        // Update next candidate prime
        prime = *candidate.iter().find(|x| **x != 0).unwrap();

        if prime > lim {
            break;
        }
    }

    let rest_prime = candidate
        .iter()
        .filter(|x| **x != 0)
        .collect::<Vec<&usize>>();

    for prime in rest_prime {
        primes.push(*prime);
    }

    primes
}

fn sieve(prime: usize, candidate: &mut [usize]) {
    for slot in candidate.iter_mut() {
        if *slot % prime == 0 {
            *slot = 0;
        }
    }
}
