use std::sync::Arc;
use futures::stream::TryStreamExt;
use axum::{extract::{Path, State}, Json};
use hyper::StatusCode;
use uuid::Uuid;

use crate::{model::User, AppState};

pub async fn get_users(State(state) : State<Arc<AppState>>) -> Json<Vec<User>> {
    let cursor = state.users.find(None, None).await.unwrap();
    let users: Vec<User> = cursor.try_collect().await.unwrap();

    Json(users)
}

pub async fn get_user_by_id(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Json<Option<User>> {
    let filter = mongodb::bson::doc! {"_id": &id};
    let user = state.users.find_one(filter, None).await.unwrap();

    Json(user)
}

pub async fn create_user(State(state): State<Arc<AppState>>, Json(mut user): Json<User>) -> (StatusCode, Json<User>) {
    user.id = Some(Uuid::new_v4().to_string());

    match state.users.insert_one(&user, None).await {
        Ok(_) => (StatusCode::CREATED, Json(user)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(User {id: None, name: "".to_string(), email: "".to_string()}))
    }
}

pub async fn update_user(State(state): State<Arc<AppState>>, Path(id): Path<String>, Json(user) : Json<User>) -> (StatusCode, Json<Option<User>>) {
    let filter = mongodb::bson::doc! {"_id": &id};
    let update = mongodb::bson::doc! {"$set": {"_id": &id, "name": &user.name, "email": &user.email}};

    match state.users.find_one_and_update(filter, update, None).await {
        Ok(Some(result)) => (StatusCode::OK, Json(Some(result))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        _ => (StatusCode::NOT_FOUND, Json(None))
     }
}

pub async fn delete_user(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> StatusCode {
    let filter = mongodb::bson::doc! {"_id": &id};
    match state.users.delete_one(filter, None).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}