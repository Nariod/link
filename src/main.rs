// internal packages
pub mod routes;
pub mod server;
pub mod util;

#[macro_use]
extern crate prettytable;

#[actix_web::main]
async fn main() {
    let _ = util::cli::main_loop().await;
}
