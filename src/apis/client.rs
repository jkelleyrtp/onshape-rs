use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient {
    accounts_api: Box<dyn crate::apis::AccountsApi>,
    app_elements_api: Box<dyn crate::apis::AppElementsApi>,
    assemblies_api: Box<dyn crate::apis::AssembliesApi>,
    blob_elements_api: Box<dyn crate::apis::BlobElementsApi>,
    documents_api: Box<dyn crate::apis::DocumentsApi>,
    drawings_api: Box<dyn crate::apis::DrawingsApi>,
    elements_api: Box<dyn crate::apis::ElementsApi>,
    feature_studios_api: Box<dyn crate::apis::FeatureStudiosApi>,
    metadata_api: Box<dyn crate::apis::MetadataApi>,
    open_api_api: Box<dyn crate::apis::OpenAPIApi>,
    part_studios_api: Box<dyn crate::apis::PartStudiosApi>,
    parts_api: Box<dyn crate::apis::PartsApi>,
    translations_api: Box<dyn crate::apis::TranslationsApi>,
}

impl APIClient {
    pub fn new<C: hyper::client::Connect>(configuration: Configuration<C>) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            accounts_api: Box::new(crate::apis::AccountsApiClient::new(rc.clone())),
            app_elements_api: Box::new(crate::apis::AppElementsApiClient::new(rc.clone())),
            assemblies_api: Box::new(crate::apis::AssembliesApiClient::new(rc.clone())),
            blob_elements_api: Box::new(crate::apis::BlobElementsApiClient::new(rc.clone())),
            documents_api: Box::new(crate::apis::DocumentsApiClient::new(rc.clone())),
            drawings_api: Box::new(crate::apis::DrawingsApiClient::new(rc.clone())),
            elements_api: Box::new(crate::apis::ElementsApiClient::new(rc.clone())),
            feature_studios_api: Box::new(crate::apis::FeatureStudiosApiClient::new(rc.clone())),
            metadata_api: Box::new(crate::apis::MetadataApiClient::new(rc.clone())),
            open_api_api: Box::new(crate::apis::OpenAPIApiClient::new(rc.clone())),
            part_studios_api: Box::new(crate::apis::PartStudiosApiClient::new(rc.clone())),
            parts_api: Box::new(crate::apis::PartsApiClient::new(rc.clone())),
            translations_api: Box::new(crate::apis::TranslationsApiClient::new(rc.clone())),
        }
    }

    pub fn accounts_api(&self) -> &dyn crate::apis::AccountsApi{
        self.accounts_api.as_ref()
    }

    pub fn app_elements_api(&self) -> &dyn crate::apis::AppElementsApi{
        self.app_elements_api.as_ref()
    }

    pub fn assemblies_api(&self) -> &dyn crate::apis::AssembliesApi{
        self.assemblies_api.as_ref()
    }

    pub fn blob_elements_api(&self) -> &dyn crate::apis::BlobElementsApi{
        self.blob_elements_api.as_ref()
    }

    pub fn documents_api(&self) -> &dyn crate::apis::DocumentsApi{
        self.documents_api.as_ref()
    }

    pub fn drawings_api(&self) -> &dyn crate::apis::DrawingsApi{
        self.drawings_api.as_ref()
    }

    pub fn elements_api(&self) -> &dyn crate::apis::ElementsApi{
        self.elements_api.as_ref()
    }

    pub fn feature_studios_api(&self) -> &dyn crate::apis::FeatureStudiosApi{
        self.feature_studios_api.as_ref()
    }

    pub fn metadata_api(&self) -> &dyn crate::apis::MetadataApi{
        self.metadata_api.as_ref()
    }

    pub fn open_api_api(&self) -> &dyn crate::apis::OpenAPIApi{
        self.open_api_api.as_ref()
    }

    pub fn part_studios_api(&self) -> &dyn crate::apis::PartStudiosApi{
        self.part_studios_api.as_ref()
    }

    pub fn parts_api(&self) -> &dyn crate::apis::PartsApi{
        self.parts_api.as_ref()
    }

    pub fn translations_api(&self) -> &dyn crate::apis::TranslationsApi{
        self.translations_api.as_ref()
    }

}
