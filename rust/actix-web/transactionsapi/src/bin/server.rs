use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(transactionsapi::config))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
