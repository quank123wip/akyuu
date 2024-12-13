use sea_orm::{Database, DatabaseConnection};
use tokio::time; // Assuming you are using SeaORM for database interactions

pub mod handler; // Assuming you have a handler module for your API handlers
pub mod routes; // Assuming you have a routes module to define your API routes
//pub mod utils; // Assuming you have a utils module for utility functions or middleware
pub mod error; // Assuming you have an error module for handling errors in your API
pub mod entities; // Assuming you have a schema module for defining your database schema with SeaORM entities and enums, etc.
pub mod config;


#[tokio::main]
async fn main() {
    let config = config::get_config().unwrap();

    let state = AppState {
        db: time::timeout(time::Duration::from_secs(2 /* TODO: configurable timeout */), async {
            Database::connect(&config.db_url).await.unwrap_or_else(|err| {
                panic!("[Error] Failed to connect to the database: {}", err);
            })
        }).await.unwrap_or_else(|_err| {
            panic!("[Error] Connection to database timeout");
        }),
        config: config.clone(),
    };

    let app = routes::setup_routes(state);

    let listener = tokio::net::TcpListener::bind(&config.server_url).await.unwrap_or_else(|err| {
        panic!("[Error] Failed to listen at {} : {}", &config.server_url, err);
    });
    eprintln!("Akyuu web api endpoint 「鈴奈庵」 is running.");
    axum::serve(listener, app).await.unwrap_or_else(|err| {
        panic!("[Error] Failed to start server : {}", err);
    });
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: config::Config,
}
