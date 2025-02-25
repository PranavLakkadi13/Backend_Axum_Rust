// NOTE: This is not complete and will be done in due course

use crate::routes::validate_data_with_serde::ResponseUser;
use axum::extract::FromRequestParts;
use axum::{
    body::{Body, Bytes},
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum::{Json, RequestExt, RequestPartsExt};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    pub username: String,
    pub password: String,
}

pub async fn custom_json_extractor(Json(body): Json<RequestUser>) -> StatusCode {
    let value = body.password.len();
    if value < 8 {
        StatusCode::NOT_ACCEPTABLE
    } else {
        StatusCode::ACCEPTED
    }
}

// impl<B> FromRequest<B> for RequestUser
// where
//     B: Send + Sync
// {
//     type Rejection = (StatusCode, String);
//
//     async fn from_request(req: Request) -> Result<Self, Self::Rejection> {
//         // let Json(user) = req.extract::<Json<ResponseUser>>()
//
//         todo!()
//     }
// }
