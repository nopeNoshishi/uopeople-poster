extern crate diesel;

mod domains;
mod infrastructures;
mod presentation;
mod application;

use presentation::server;

fn main() {
    server::run();
}
