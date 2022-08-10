use crate::engine::Result;

pub mod commit;
pub mod start;

pub(crate) fn map_known_error(err: query_core::CoreError) -> Result<String> {
    let user_error: user_facing_errors::Error = err.into();
    let value = serde_json::to_string(&user_error)?;

    Ok(value)
}
