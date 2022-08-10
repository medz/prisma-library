use datamodel_connector::Diagnostics;
use libc::c_char;
use query_connector::error::ConnectorError;
use query_core::CoreError;

use crate::string_to_c_char;

#[repr(C)]
pub enum ApiError {
    Conversion(*const c_char, *const c_char),
    Configuration(*const c_char),
    Core(*const c_char),
    Connector(*const c_char),
    AlreadyConnected,
    NotConnected,
    JsonDecode(*const c_char),
}

impl ApiError {
    pub fn conversion(diagnostics: Diagnostics, dml: impl ToString) -> Self {
        let msg = diagnostics.errors().first().unwrap().message();
        let msg = string_to_c_char(&msg);
        let dml = string_to_c_char(&dml.to_string());

        ApiError::Conversion(msg, dml)
    }

    pub fn configuration(msg: impl ToString) -> Self {
        ApiError::Configuration(string_to_c_char(&msg.to_string()))
    }
}

impl From<user_facing_errors::Error> for ApiError {
    fn from(err: user_facing_errors::Error) -> Self {
        let msg = err.message();
        let msg = string_to_c_char(&msg);
        ApiError::Core(msg)
    }
}

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        match err {
            CoreError::ConfigurationError(message) => Self::configuration(&message),
            core_error => Self::Core(string_to_c_char(format!("{:?}", core_error).as_str())),
        }
    }
}

impl From<ConnectorError> for ApiError {
    fn from(err: ConnectorError) -> Self {
        let known_error = err.user_facing_error;
        if known_error.is_some() {
            let err = known_error.unwrap();
            let err = err.message;

            return Self::Connector(string_to_c_char(&err));
        }

        let err = err.kind;
        let err = format!("{:?}", err);
        let err = string_to_c_char(&err);

        Self::Connector(err)
    }
}

impl From<url::ParseError> for ApiError {
    fn from(e: url::ParseError) -> Self {
        Self::configuration(format!("Error parsing connection string: {}", e))
    }
}

impl From<connection_string::Error> for ApiError {
    fn from(e: connection_string::Error) -> Self {
        Self::configuration(format!("Error parsing connection string: {}", e))
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        let e = format!("{}", e);
        let e = string_to_c_char(&e);
        Self::JsonDecode(e)
    }
}
