mod handlers;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

use rusty_tasks_api::{config::Config, database::TaskService};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    dotenv().ok();
    
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");

    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("❌ Failed to connect to the database");

    // Create task service
    let task_service = TaskService::new(pool);

    println!("🚀 Starting server at http://{}:{}", config.server_host, config.server_port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Cors::permissive()) // Allow all origins during development
            .app_data(web::Data::new(task_service.clone()))
            .configure(handlers::configure_routes)
    })
    .bind((config.server_host.as_str(), config.server_port))?
    .run()
    .await
}
