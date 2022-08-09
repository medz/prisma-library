use std::sync::Arc;

use datamodel_connector::ConnectorCapabilities;
use libc::c_char;
use prisma_models::InternalDataModelBuilder;
use query_core::{schema::QuerySchemaRef, schema_builder};
use request_handlers::dmmf;

use crate::{c_char_to_string, string_to_c_char};

/// Dmmf parsed result.
#[repr(C)]
pub enum DMMF {
    /// Dmmf parsed result.
    Document(*const c_char),

    /// Error message.
    Error(*const c_char),
}

/// Get DMMF json string from the schema.
#[no_mangle]
pub unsafe extern "C" fn dmmf(datamodel_string: *const c_char) -> DMMF {
    // Read the datamodel string.
    let datamodel_string = c_char_to_string(datamodel_string);
    let datamodel = datamodel::parse_datamodel(&datamodel_string);
    match datamodel {
        Ok(datamodel) => {
            let config = datamodel::parse_configuration(&datamodel_string);
            match config {
                Ok(config) => {
                    let datasource = config.subject.datasources.first();
                    let capabilities = datasource
                        .map(|ds| ds.capabilities())
                        .unwrap_or_else(ConnectorCapabilities::empty);

                    let referential_integrity = datasource
                        .map(|ds| ds.referential_integrity())
                        .unwrap_or_default();
                    let internal_data_model =
                        InternalDataModelBuilder::from(&datamodel.subject).build("".into());

                    let query_schema: QuerySchemaRef = Arc::new(schema_builder::build(
                        internal_data_model,
                        true,
                        capabilities,
                        config.subject.preview_features().iter().collect(),
                        referential_integrity,
                    ));
                    let dmmf = dmmf::render_dmmf(&datamodel.subject, query_schema);
                    let dmmf = serde_json::to_string(&dmmf);
                    match dmmf {
                        Ok(dmmf) => {
                            let dmmf = string_to_c_char(&dmmf);
                            DMMF::Document(dmmf)
                        }
                        Err(err) => {
                            let err = string_to_c_char(&err.to_string());
                            DMMF::Error(err)
                        }
                    }
                }
                Err(err) => {
                    let err = err.errors().first().unwrap();
                    return DMMF::Error(string_to_c_char(err.message()));
                }
            }
        }
        Err(err) => {
            let err = err.errors().first().unwrap();
            return DMMF::Error(string_to_c_char(err.message()));
        }
    }
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
        let datamodel_string = string_to_c_char(datamodel_string);
        let dmmf = unsafe { dmmf(datamodel_string) };
        match dmmf {
            DMMF::Document(dmmf) => {
                let dmmf = c_char_to_string(dmmf);
                println!("{}", dmmf);
                assert!(&dmmf.contains("\"name\":\"User\""));
            }
            DMMF::Error(error) => {
                let error = c_char_to_string(error);
                panic!("{}", error);
            }
        }

        // let schema = schema.as_ptr();
        // let result = unsafe { dmmf(schema) };
        // match result {
        //     DMMF::Document(json) => {
        //         let json = unsafe { CStr::from_ptr(json) };
        //         let json = json.to_str().unwrap();

        //         assert!(json.contains("\"name\":\"User\""));
        //     },
        //     DMMF::Error(_) => todo!(),
        // }
    }
}
