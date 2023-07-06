use axum::{Extension, Json};

use crate::api::http::{error::HttpApiError, models::Path, HttpApiHandler, HttpApiResult};

pub(crate) async fn dump(
    Json(_path): Json<Path>,
    Extension(_state): Extension<HttpApiHandler>,
) -> HttpApiResult<()> {
    Err(HttpApiError::GeneralError)
}

pub(crate) async fn load(
    Json(_path): Json<Path>,
    Extension(_state): Extension<HttpApiHandler>,
) -> HttpApiResult<()> {
    Err(HttpApiError::PathNotFound)
}