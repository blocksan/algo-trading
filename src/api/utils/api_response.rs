use serde::Serialize;
use actix_web::HttpResponse;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

impl ApiResponse<serde_json::Value> {
    pub fn success(message: impl Into<String>, data: serde_json::Value) -> HttpResponse {
        HttpResponse::Ok().json(Self {
            status: "success".to_string(),
            message: message.into(),
            data: Some(data),
        })
    }

    pub fn error(message: impl Into<String>) -> HttpResponse {
        HttpResponse::BadRequest().json(Self {
            status: "error".to_string(),
            message: message.into(),
            data: None,
        })
    }
}