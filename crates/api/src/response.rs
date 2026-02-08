use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<ApiError>,
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub id: String,
    pub message: String,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ID: {}, Message: {}", self.id, self.message)
    }
}
