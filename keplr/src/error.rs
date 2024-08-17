#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("JavaScript Error: {0}")]
    JsError(String),

    #[error("Serialization Error: {0}")]
    SerializationError(#[from] serde_wasm_bindgen::Error),

    #[error("Unknown Error")]
    Unknown,
}

impl From<js_sys::Error> for Error {
    fn from(error: js_sys::Error) -> Self {
        let message = error
            .message()
            .as_string()
            .unwrap_or("unknown JS error".to_string());
        Error::JsError(message)
    }
}
