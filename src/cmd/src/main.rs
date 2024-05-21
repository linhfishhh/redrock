mod settings;

use std::rc::Rc;
use axum::http::Method;
use presentation::config::configure;
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use log::info;
use tower::balance::pool::Pool;
use crate::settings::Settings;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let settings = Settings::from_env().unwrap();

    let pool = settings.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    infra::config::configure(&pool).await.unwrap();

    let route = configure(&pool);

    info!("starting server listening on: {}", &settings.host);
    let listener = tokio::net::TcpListener::bind(&settings.host).await.unwrap();

    axum::serve(listener, route).await.unwrap();
}
