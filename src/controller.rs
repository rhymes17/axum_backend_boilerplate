use std::sync::Arc;
use futures:: TryStreamExt;


use axum::{extract::State, Json};

use crate::{model::User, AppState};

pub async fn get_users(State(state) : State<Arc<AppState>>) -> Json<Vec<User>> {
    let  cursor = state.users.find(None, None).await.unwrap();
    let users = cursor.try_collect().await.unwrap();
    Json(users)
}