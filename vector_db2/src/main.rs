use anyhow::{Result, anyhow};
use qdrant_client::prelude::*;
use qdrant_client::qdrant::{
    CreateCollection, SearchPoints, VectorParams, VectorsConfig, Distance,
};
use rand::prelude::*;
use serde_json::json;
use qdrant_client::qdrant::vectors_config::Config;
use tokio;


#[tokio::main]
async fn main() -> Result<()> {
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = QdrantClient::new(Some(config))?;

    let collection_name = "test_collection";
    let _ = client.delete_collection(collection_name).await;

    client.create_collection(&CreateCollection {
        collection_name: collection_name.into(),
        vectors_config: Some(VectorsConfig {
            config: Some(Config::Params(VectorParams {
                size: 4,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })),
        }),
        ..Default::default()
    }).await?;

    let mut rng = rand::thread_rng();
    // Ingest multiple points with varied data into the database
    for i in 0..5 {
        let vector: Vec<f32> = (0..4).map(|_| rng.gen::<f32>()).collect();
        println!("Vector: {:?}", vector);
        let payload = json!({
            "id": {"id": format!("Info{}", i)},
            "isDiabetic": format!("isDiabetic{}", rng.gen_range(0..2)),
            "age": rng.gen_range(20..50),
        }).try_into().map_err(|e| anyhow!("Payload conversion error: {:?}", e))?;

        let points = vec![PointStruct::new(i, vector, payload)];
        client.upsert_points(collection_name, None, points, None).await?;
    }

    let search_result = client.search_points(&SearchPoints {
        collection_name: collection_name.into(),
        vector: vec![0.15, 0.25, 0.35, 0.45],
        filter: None,
        limit: 5,
        with_payload: Some(true.into()),
        ..Default::default()
    }).await?;

    // visualize ages of all points
    for (index, point) in search_result.result.iter().enumerate() {
        println!("Point {} Age: {:?}", index + 1, point.payload["age"]);
        
    }
    // visualize isDiabetic of all points
    for (index, point) in search_result.result.iter().enumerate() {
        println!("Point {} isDiabetic: {:?}", index + 1, point.payload["isDiabetic"]);
    }
    Ok(())
}


