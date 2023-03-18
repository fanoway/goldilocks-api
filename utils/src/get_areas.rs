use dotenv::dotenv;
use gql_client::{Client, ClientConfig};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct Area {
    area_name: String,
    metadata: Metadata,
}

#[derive(Deserialize, Serialize, Debug)]
struct Metadata {
    lat: f64,
    long: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
    areas: NodeList<Area>,
}

#[derive(Deserialize, Serialize, Debug)]
struct NodeList<T> {
    data: Vec<T>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables
    dotenv().ok();

    // Connect to redis to write data to it
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set.");
    let redis_client = match redis::Client::open(redis_url) {
        Ok(val) => Ok(val),
        Err(val) => {
            println!("{}", val.to_string());
            Err(val.to_string())
        }
    }?;

    let redis_connection = redis_client.get_connection()?;

    let query = "query Locations {
        areas {
          area_name
            metadata {
              lat
              lng
            }
          }
      }
      ";

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
            None => Err("Error".to_string()),
        },
        Err(val) => {
            println!("{}", val.to_string());
            Err(val.to_string())
        }
    }?;

    for area in &response.areas.data {
        println!(
            "{} | {} | {}",
            area.area_name, area.metadata.lat, area.metadata.long
        );
    }

    Ok(())
}
