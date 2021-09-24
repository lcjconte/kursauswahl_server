use tokio::time::{sleep, Duration};
use std::env;
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use rocket::{Route, response};

pub fn get_routes() -> Vec<Route> {
    routes![wait10, course]
}

#[get("/wait10")]
pub async fn wait10() -> &'static str{
    sleep(Duration::new(10, 0)).await;
    "Finished"
}

#[get("/course")]
pub async fn course() -> response::status::Accepted<String> {
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    dotenv().ok();
    let connurl = env::var("DATABASE_URL").unwrap();
    let (client, connection) =
        tokio_postgres::connect(&connurl, connector).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    let rows = client
        .query("SELECT * FROM testable", &[])
        .await.unwrap();
    let value: i32 = rows[0].get(0);
    response::status::Accepted(Some(value.to_string()))
}