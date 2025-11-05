use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_cors::Cors;
use config::Config;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json; // <-- This is the fix

#[derive(Clone, Debug)]
pub struct AppState {
    pub linera_rpc_url: String,
    pub application_id: String,
    // Will be used in future waves for AI integration
    pub ai_config: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct CreateArenaRequest {
    question: String,
    outcomes: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct PlacePredictionRequest {
    arena_id: u64,
    outcome_index: usize,
    amount: u64,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "rust-orchestrator"
    }))
}

async fn create_arena(
    _data: web::Json<CreateArenaRequest>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse> {
    // TODO: Implement Linera transaction submission
    // This will be connected in Wave 2
    log::info!("Create arena request received");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Arena creation will be implemented in Wave 2"
    })))
}

async fn place_prediction(
    _data: web::Json<PlacePredictionRequest>,
    _app_state: web::Data<AppState>,
) -> Result<HttpResponse> {
    // TODO: Implement Linera transaction submission
    log::info!("Place prediction request received");
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success", 
        "message": "Prediction placement will be implemented in Wave 2"
    })))
}

async fn get_arenas(_app_state: web::Data<AppState>) -> Result<HttpResponse> {
    // TODO: Implement Linera GraphQL query
    log::info!("Get arenas request received");
    
    Ok(HttpResponse::Ok().json(serde_json::json!([])))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    // Load configuration
    let settings = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()
        .expect("Failed to load configuration");
    
    let port = settings.get_int("port").unwrap_or(8081) as u16;
    let linera_rpc_url = settings.get_string("linera_rpc_url")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());
    let application_id = settings.get_string("application_id")
        .unwrap_or_else(|_| "".to_string());
    
    let app_state = web::Data::new(AppState {
        linera_rpc_url,
        application_id,
        ai_config: HashMap::new(),
    });

    log::info!("Starting AetherArena Orchestrator on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .route("/health", web::get().to(health_check))
            .route("/api/arenas", web::get().to(get_arenas))
            .route("/api/arenas/create", web::post().to(create_arena))
            .route("/api/predictions/place", web::post().to(place_prediction))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}