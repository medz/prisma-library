use crate::{c_char_to_string, error::ApiError, string_to_c_char};
use datamodel_connector::ConnectorCapabilities;
use libc::c_char;
use prisma_models::InternalDataModelBuilder;
use query_core::{schema::QuerySchemaRef, schema_builder};
use request_handlers::dmmf;
use std::sync::Arc;

type Result<T> = std::result::Result<T, ApiError>;

/// @See https://github.com/prisma/prisma-engines/blob/main/query-engine/query-engine-node-api/src/functions.rs#L29
fn inner_dmmf_parser(datamodel_string: String) -> Result<String> {
    let datamodel = datamodel::parse_datamodel(&datamodel_string)
        .map_err(|errors| ApiError::conversion(errors, &datamodel_string))?;

    let config = datamodel::parse_configuration(&datamodel_string)
        .map_err(|errors| ApiError::conversion(errors, &datamodel_string))?;
    let datasource = config.subject.datasources.first();

    let capabilities = datasource
        .map(|ds| ds.capabilities())
        .unwrap_or_else(ConnectorCapabilities::empty);

    let referential_integrity = datasource
        .map(|ds| ds.referential_integrity())
        .unwrap_or_default();

    let internal_data_model = InternalDataModelBuilder::from(&datamodel.subject).build("".into());

    let query_schema: QuerySchemaRef = Arc::new(schema_builder::build(
        internal_data_model,
        true,
        capabilities,
        config.subject.preview_features().iter().collect(),
        referential_integrity,
    ));

    let dmmf = dmmf::render_dmmf(&datamodel.subject, query_schema);

    Ok(serde_json::to_string(&dmmf)?)
}

/// Get DMMF json string from the schema.
#[no_mangle]
pub unsafe extern "C" fn dmmf(
    datamodel_string: *const c_char,
    error: extern "C" fn(ApiError),
    done: extern "C" fn(*const c_char),
) {
    let datamodel_string = c_char_to_string(datamodel_string);
    let dmmf = inner_dmmf_parser(datamodel_string);

    match dmmf {
        Ok(dmmf) => {
            let dmmf_string = string_to_c_char(&dmmf);
            done(dmmf_string);
        }
        Err(err) => {
            error(err);
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dmmf() {
        let datamodel_string = r#"
generator client {
    provider = "prisma-dart-client"
}

datasource db {
    provider = "postgresql"
    url      = env("DATABASE_URL")
}

model User {
    id          Int @id @default(autoincrement())
    name        String
    createdAt   DateTime @default(now())
}
          "#;
        let dmmf_string = inner_dmmf_parser(datamodel_string.to_string());
        assert!(dmmf_string.is_ok());

        extern "C" fn error(_err: ApiError) {
            assert!(false);
        }

        extern "C" fn done(dmmf: *const c_char) {
            let dmmf = c_char_to_string(dmmf);
            assert!(dmmf.contains("\"name\":\"User\""));
        }

        let c_char_datamodel = string_to_c_char(datamodel_string);
        unsafe {
            dmmf(c_char_datamodel, error, done);
        }
    }
}
