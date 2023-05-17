extern crate diesel;

mod application;
mod domains;
mod infrastructures;
mod presentation;

use presentation::server;

fn main() {
    server::run();
}
