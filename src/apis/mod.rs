use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    UriError(hyper::error::UriError),
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

mod request;

mod accounts_api;
pub use self::accounts_api::{ AccountsApi, AccountsApiClient };
mod app_elements_api;
pub use self::app_elements_api::{ AppElementsApi, AppElementsApiClient };
mod assemblies_api;
pub use self::assemblies_api::{ AssembliesApi, AssembliesApiClient };
mod blob_elements_api;
pub use self::blob_elements_api::{ BlobElementsApi, BlobElementsApiClient };
mod documents_api;
pub use self::documents_api::{ DocumentsApi, DocumentsApiClient };
mod drawings_api;
pub use self::drawings_api::{ DrawingsApi, DrawingsApiClient };
mod elements_api;
pub use self::elements_api::{ ElementsApi, ElementsApiClient };
mod feature_studios_api;
pub use self::feature_studios_api::{ FeatureStudiosApi, FeatureStudiosApiClient };
mod metadata_api;
pub use self::metadata_api::{ MetadataApi, MetadataApiClient };
mod open_api_api;
pub use self::open_api_api::{ OpenAPIApi, OpenAPIApiClient };
mod part_studios_api;
pub use self::part_studios_api::{ PartStudiosApi, PartStudiosApiClient };
mod parts_api;
pub use self::parts_api::{ PartsApi, PartsApiClient };
mod translations_api;
pub use self::translations_api::{ TranslationsApi, TranslationsApiClient };

pub mod configuration;
pub mod client;
