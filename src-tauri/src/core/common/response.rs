use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success_with_data(data: T) -> Self {
        Self {
            code: 10000,
            msg: "success".into(),
            data: Some(data),
        }
    }

    pub fn success_with_msg() -> Self {
        Self {
            code: 10000,
            msg: "success".into(),
            data: None,
        }
    }
}

impl<T> ApiResponse<T> {
    pub fn error(msg: &str) -> Self {
        Self {
            code: 10001,
            msg: msg.into(),
            data: None,
        }
    }
}
