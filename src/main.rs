use std::{env, sync::Arc};

use axum::{routing::{post, get}, Router};
use controller::{create_user, get_user_by_id, get_users};
use dotenvy::dotenv;
use model::User;
use mongodb::{options::ClientOptions, Client, Collection};
use tokio::net::TcpListener;

mod model;
mod controller;

#[derive(Clone)]
struct AppState{
    users: Collection<User>
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mongo_uri = env::var("MONGO_URI").expect("Unable to get MONGO_URI from env");
    let database_name = env::var("DATABASE_NAME").expect("Unable to get DATABASE_NAME from env");
    let collection_name = "users";

    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database(&database_name);
    let user_collection = database.collection::<User>(&collection_name);

    let app_state = Arc::new(AppState{users: user_collection});

    // Router/App
    let app: Router = Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/user/{id}", get(get_user_by_id))
        .with_state(app_state);

    // Listener
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
