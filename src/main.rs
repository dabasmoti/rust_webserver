use actix_web::{web, App, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    instances: Vec<Instance>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Instance {
    _id: String,
}

#[derive(Serialize)]
struct PredictionResult {
    predictions: Value,
}

async fn make_prediction(
    data: web::Data<Arc<RwLock<Map<String, Value>>>>,
    body: web::Json<RequestBody>,
) -> Result<HttpResponse> {
    let map = data.read().unwrap();
    let mut prediction = Value::Null;
    
    for instance in &body.instances {
        prediction = map.get(&instance._id)
                        .cloned()
                        .unwrap_or_else(|| Value::String(format!("error: user not found: {}", instance._id)));
    }

    Ok(HttpResponse::Ok().json(PredictionResult { predictions: prediction }))
}

pub struct JsonParser;

impl JsonParser {
    pub fn read_json(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&data)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hash_map = JsonParser::read_json("predic.json").unwrap();
    let data = Arc::new(RwLock::new(hash_map));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::clone(&data))) // Use .app_data instead of .data
            .app_data(web::JsonConfig::default().limit(4096)) // limit size of the payload (optional)
            .route("/predict", web::post().to(make_prediction))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}