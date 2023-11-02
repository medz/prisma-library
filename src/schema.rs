// use std::ffi::{CString, CStr};

// use crate::{error::PrismaError, result::PrismaResult};

// /// Format a Prisma schema.
// /// 
// /// # Arguments
// /// 
// /// - `schema` - The Prisma schema to format.
// /// - `params` - The formatting parameters as a JSON string. 
// ///         The following parameters are [prisma-format](https://github.com/prisma/prisma-engines/blob/main/prisma-fmt/src/lib.rs#L53) source.
// #[no_mangle]
// pub extern "C" fn prisma_schema_format(
//     schema: *const libc::c_char,
//     params: *const libc::c_char,
// ) -> *const libc::c_char {
//     let schema = unsafe { CStr::from_ptr(schema) };
//     let schema = schema.to_str().unwrap();

//     let params = unsafe { CStr::from_ptr(params) };
//     let params = params.to_str().unwrap();

//     let formated = prisma_fmt::format(schema, params);
//     let formated = CString::new(formated).unwrap();

//     formated.as_ptr()
// }

// #[repr(C)]
// pub enum PrismaSchemaValidateResult {
//     Ok,
//     Err { err: *const libc::c_char },
// }

// /// Validate a Prisma schema.
// #[no_mangle]
// pub extern "C" fn prisma_schema_validate(
//     schema: *const libc::c_char,
// ) -> PrismaSchemaValidateResult {
//     let schema = unsafe { CStr::from_ptr(schema) };
//     let schema  = schema.to_str().unwrap().to_string();

//     let errors = prisma_fmt::validate(schema);

//     match errors {
//         Ok(_) => PrismaSchemaValidateResult::Ok,
//         Err(err) => {
//             let err = CString::new(err.to_string()).unwrap();

//             PrismaSchemaValidateResult::Err { err: err.as_ptr() }
//         }
//     }
// }

// /// Get the Prisma schema configuration.
// /// 
// /// Docs: https://prisma.github.io/prisma-engines/doc/prisma_fmt/fn.get_config.html
// #[no_mangle]
// pub extern "C" fn prisma_schema_config(params: *const libc::c_char) -> PrismaResult<*const libc::c_char> {
//     let params = unsafe { CStr::from_ptr(params) };
//     let params = params.to_str().unwrap();

//     let config = prisma_fmt::get_config(params.to_string());
    
//     match config {
//         Ok(config) => {
//             let config = CString::new(config).unwrap();

//             PrismaResult::Ok(config.as_ptr())
//         },
//         Err(err) => {
//             let err = CString::new(err).unwrap();

//             PrismaResult::Err(PrismaError::JsonDecode(err.as_ptr()))
//         }
//     }
// }

// /// Get Prisma schema DMMF.
// /// 
// /// Docs: https://prisma.github.io/prisma-engines/doc/prisma_fmt/fn.get_dmmf.html
// #[no_mangle]
// pub extern "C" fn prisma_schema_dmmf(schema: *const libc::c_char) -> PrismaResult<*const libc::c_char> {
//     let schema = unsafe { CStr::from_ptr(schema) };
//     let schema = schema.to_str().unwrap();

//     let dmmf = prisma_fmt::get_dmmf(schema.to_string());
    
//     match dmmf {
//         Ok(dmmf) => {
//             let dmmf = CString::new(dmmf).unwrap();

//             PrismaResult::Ok(dmmf.as_ptr())
//         },
//         Err(err) => {
//             let err = CString::new(err).unwrap();

//             PrismaResult::Err(PrismaError::JsonDecode(err.as_ptr()))
//         }
//     }
// }