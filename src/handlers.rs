use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Form,
};
use serde::Deserialize;

use crate::state::ServiceState;

#[derive(Deserialize, Debug)]
pub struct PutForm {
    value: String,
}

pub async fn put(
    State(state): State<Arc<ServiceState>>,
    Path(key): Path<String>,
    Form(form): Form<PutForm>,
) {
    state.store.pin().insert(key, form.value);
}

pub async fn get(
    State(state): State<Arc<ServiceState>>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    match state.store.pin().get(&key) {
        Some(value) => Ok(value.to_string()),
        None => Err(StatusCode::NOT_FOUND),
    }
}
