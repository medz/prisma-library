use super::{
    core::{async_panic_to_error, Engine, EngineBuilder, Inner},
    instance, Result,
};
use crate::{error::ApiError, string_to_c_char};

impl Engine {
    /// Disconnect and drop the core. Can be reconnected later with `#connect`.
    pub async fn disconnect(&self) -> Result<()> {
        async_panic_to_error(async {
            let mut inner = self.inner.write().await;
            let engine = inner.as_engine()?;

            let config = datamodel::parse_configuration(&engine.datamodel.raw)
                .map_err(|errors| ApiError::conversion(errors, &engine.datamodel.raw))?;

            let builder = EngineBuilder {
                datamodel: engine.datamodel.clone(),
                config,
            };

            *inner = Inner::Builder(builder);

            Ok(())
        })
        .await
    }
}

#[no_mangle]
pub extern "C" fn disconnect(
    id: i64,
    error: extern "C" fn(error: ApiError),
    done: extern "C" fn(),
) {
    let lock = instance::lock();
    let engine = lock.get(id.unsigned_abs());

    if let Some(engine) = engine {
        let result = futures::executor::block_on(engine.disconnect());
        if result.is_ok() {
            done();
        } else {
            error(result.err().unwrap());
        }
    } else {
        let err = "Engine not found";
        let err = string_to_c_char(err);
        error(ApiError::Connector(err));
    }
}
