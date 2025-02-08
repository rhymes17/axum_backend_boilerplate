use axum::{routing::{get, post, delete, put}, Router};
use controller::{create_user, get_users};
use model::User;
use mongodb::{Client, options::ClientOptions, Collection};
use tokio::net::TcpListener;
use std::{env, sync::Arc};
use dotenvy::dotenv;

mod model;
mod controller;

#[derive(Clone)]
struct AppState {
    users: Collection<User>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("unable to get MONGO_URI from env");
    let database_name = env::var("DATABASE_NAME").expect("unable to get DATABASE_NAME from env");
    let collection_name = env::var("COLLECTION_NAME").expect("unable to get COLLECTION_NAME from the env");

    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database(&database_name);
    let user_collection = database.collection::<User>(&collection_name);

    let app_state = Arc::new(AppState{users: user_collection});

    let app = Router::new()
        .route("/users", post(create_user).get(get_users)).with_state(app_state);

    // Create Listener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}