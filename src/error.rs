use std::{collections::HashMap, ffi::CString};

pub enum PrismaError {
    AlreadyConnected,
    NotConnected,
    Json(String),
    Configuration(String),
}

impl PrismaError {
    pub fn to_json(self) -> HashMap<String, String> {
        let (code, message, data) = match self {
            PrismaError::AlreadyConnected => ("AlreadyConnected", "Already connected to database", None),
            PrismaError::NotConnected => ("NotConnected", "Not connected to database", None),
            PrismaError::Json(json) => ("Json", "", Some(json)),
            PrismaError::Configuration(message) => ("Configuration", message, None),
        };

        let mut map = HashMap::new();
        map.insert("code", code);
        map.insert("message", message);

        if let Some(data) = data {
            map.insert("data", &data);
        }

        map.into()
    }

    pub fn to_c_char(&self) -> *const libc::c_char {
        let result = serde_json::to_string(&self.to_json()).unwrap();
        let result = CString::new(serialized).unwrap();

        result.into_raw()
    }
}
