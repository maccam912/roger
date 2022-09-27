use axum::{Router, routing::get, Extension};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use eyre::Result;
// use tracing::info;

mod config;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Backend {
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Service {
    inputs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Graph {
    backends: HashMap<String, Backend>,
    services: HashMap<String, Service>,
}

impl Graph {
    fn load(path: &str) -> Result<Self> {
        let data = std::fs::read_to_string(path)?;

        let graph: Graph = toml::from_str(&data)?;
        Ok(graph)
    }
}

async fn handler(Extension(g): Extension<Graph>) -> String {
    serde_json::to_string(&g).unwrap()
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = config::init_tracing();
    // TODO if path doesn't exist need to handle failure
    let g = Graph::load("graph.toml")?;

    let app = Router::new().route("/", get(handler)).layer(Extension(g));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
