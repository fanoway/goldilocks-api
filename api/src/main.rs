mod weather_data_model;
use axum::{
    extract::{Path, Query},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Get env vars
    dotenv().ok();

    let app = Router::new().route("/", get(process_weather));

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    axum::Server::bind(std::env::var("AXUM_HOST").unwrap().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(Deserialize, Debug)]
struct LatLngParams {
    lat: f64,
    lng: f64,
}

async fn process_weather(Query(latlng): Query<LatLngParams>) -> Result<(), String> {
    // Early exit code for debug purposes
    return Err(format!("{:?}", latlng));

    let weather_raw = match weather_data_model::get_weather_from_api(latlng.lat, latlng.lng).await {
        Ok(val) => Ok(val),
        Err(_) => Err("Error parsing json".to_string()),
    }?;
    match weather_data_model::add_weather_to_db(weather_raw).await {
        Ok(_) => Ok(()),
        Err(_) => Err("Error writing to db".to_string()),
    }
}
