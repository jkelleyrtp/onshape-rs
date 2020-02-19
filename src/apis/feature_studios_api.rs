/*
 * Onshape REST API
 *
 * The Onshape REST API consumed by all clients.
 *
 * The version of the OpenAPI document: 1.104
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use serde_json;
use futures::Future;

use super::{Error, configuration};
use super::request as __internal_request;

pub struct FeatureStudiosApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> FeatureStudiosApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> FeatureStudiosApiClient<C> {
        FeatureStudiosApiClient {
            configuration,
        }
    }
}

pub trait FeatureStudiosApi {
    fn create_feature_studio(&self, did: &str, wid: &str, bt_model_element_params: crate::models::BtModelElementParams) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn get_feature_studio_contents(&self, did: &str, wvm: &str, wvmid: &str, eid: &str) -> Box<dyn Future<Item = crate::models::BtFeatureStudioContents, Error = Error<serde_json::Value>>>;
    fn get_feature_studio_specs(&self, did: &str, wvm: &str, wvmid: &str, eid: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn update_feature_studio_contents(&self, did: &str, wvm: &str, wvmid: &str, eid: &str, body: Option<&str>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>FeatureStudiosApi for FeatureStudiosApiClient<C> {
    fn create_feature_studio(&self, did: &str, wid: &str, bt_model_element_params: crate::models::BtModelElementParams) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/featurestudios/d/{did}/w/{wid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wid".to_string(), wid.to_string());
        req = req.with_body_param(bt_model_element_params);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn get_feature_studio_contents(&self, did: &str, wvm: &str, wvmid: &str, eid: &str) -> Box<dyn Future<Item = crate::models::BtFeatureStudioContents, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_feature_studio_specs(&self, did: &str, wvm: &str, wvmid: &str, eid: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid}/featurespecs".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn update_feature_studio_contents(&self, did: &str, wvm: &str, wvmid: &str, eid: &str, body: Option<&str>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/featurestudios/d/{did}/{wvm}/{wvmid}/e/{eid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_body_param(body);
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

}