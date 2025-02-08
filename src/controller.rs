use std::sync::Arc;
use futures:: TryStreamExt;
use axum::{extract::State, Json};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{model::User, AppState};

pub async fn get_users(State(state) : State<Arc<AppState>>) -> Json<Vec<User>> {
    let  cursor = state.users.find(None, None).await.unwrap();
    let users = cursor.try_collect().await.unwrap();
    Json(users)
}

pub async fn create_user(State(state): State<Arc<AppState>>, Json(mut user) : Json<User>) -> (StatusCode, Json<User>) {
    user.id = Some(Uuid::new_v4().to_string());

    match state.users.insert_one(&user, None).await {
        Ok(_) => (StatusCode::CREATED, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User {id: None, name: "".to_string(), email: "".to_string()}))
    }
}