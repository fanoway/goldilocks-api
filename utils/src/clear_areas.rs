use dotenv::dotenv;
use gql_client::{Client, ClientConfig};
use redis::Commands;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
struct Area {
    area_name: String,
    metadata: Metadata,
}

#[derive(Deserialize, Serialize, Debug)]
struct Metadata {
    lat: f64,
    lng: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    areas: Vec<Area>,
}

async fn fetch_areas(con: &mut redis::Connection) -> redis::RedisResult<Vec<String>> {
    let keys = con.keys("*")?;
    Ok(keys)
}

async fn delete_area(con: &mut redis::Connection, area: String) -> redis::RedisResult<()> {
    let _: () = con.del(&area)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables
    dotenv().ok();

    // Connect to redis so we can write data to it
    let redis_url = std::env::var("REDIS").expect("REDIS_URL must be set.");
    let redis_client = match redis::Client::open(redis_url) {
        Ok(val) => Ok(val),
        Err(val) => {
            println!("{}", val.to_string());
            Err(val.to_string())
        }
    }?;

    let mut redis_connection = redis_client.get_connection()?;

    // get all areas from redis

    let areas = fetch_areas(&mut redis_connection).await?;

    //  Delete each area

    for area in areas {
        delete_area(&mut redis_connection, area).await?;
    }

    Ok(())
}
