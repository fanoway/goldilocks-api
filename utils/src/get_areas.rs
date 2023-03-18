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

// #[derive(Deserialize, Serialize, Debug)]
// struct Areas<T> {
//     areas: Vec<T>,
// }

async fn add_area(con: &mut redis::Connection, area: &Area) -> redis::RedisResult<()> {
    let _: () = con.set(
        &area.area_name,
        redis::ToRedisArgs::to_redis_args(&json!(area.metadata).to_string()),
    )?;
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

    // Get areas from openbeta
    let query = r#"query Locations {
        areas {
          area_name
            metadata {
              lat
              lng
            }
          }
      }
      "#;

    let client_config = ClientConfig {
        endpoint: "https://api.openbeta.io".to_string(),
        timeout: Some(120),
        headers: None,
        proxy: None,
    };
    let client = Client::new_with_config(client_config);

    let response = match client.query::<Data>(query).await {
        Ok(option) => match option {
            Some(val) => Ok(val),
            None => Err("Error1".to_string()),
        },
        Err(val) => {
            // println!("{}", val.to_string());
            Err(val.to_string())
        }
    }?;

    for area in &response.areas {
        add_area(&mut redis_connection, area).await?;
        // println!(
        //     "{} | {} | {}",
        //     area.area_name, area.metadata.lat, area.metadata.lng
        // );
    }

    Ok(())
}
