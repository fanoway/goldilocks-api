use dotenv::dotenv;
use gql_client::{Client, ClientConfig};
use mongodb::options::ClientOptions;
use serde::{Deserialize, Serialize};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load env variables
    dotenv().ok();

    // Get areas from openbeta
    let query = r#"query get_areas {
        areas {
          area_name
            metadata {
              lat
              lng
            }
          }
        
      }"#;

    let client_config = ClientConfig {
        endpoint: "https://api.openbeta.io".to_string(),
        timeout: Some(120),
        headers: None,
        proxy: None,
    };
    let gql_client = Client::new_with_config(client_config);

    let response = match gql_client.query::<Data>(query).await {
        Ok(option) => match option {
            Some(val) => Ok(val),
            None => Err("Error1".to_string()),
        },
        Err(val) => Err(val.to_string()),
    }?;

    // Connect to mongodb so we can write data to it
    let connection_string = std::env::var("MONGO").expect("MONGO must be set as an env var.");
    let database: String = std::env::var("DATABASE").expect("test");
    let mongo_client_options = ClientOptions::parse(connection_string).await?;
    // Get handle
    let mongo_client = mongodb::Client::with_options(mongo_client_options)?;

    let db = mongo_client.database(&database);
    let collection = db.collection::<Area>("areas");
    collection.insert_many(&response.areas, None).await?;

    // for area in &response.areas {
    //     println!(
    //         "{} | {} | {}",
    //         area.area_name, area.metadata.lat, area.metadata.lng
    //     );
    // }

    Ok(())
}
