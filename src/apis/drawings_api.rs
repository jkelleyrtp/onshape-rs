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

pub struct DrawingsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> DrawingsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> DrawingsApiClient<C> {
        DrawingsApiClient {
            configuration,
        }
    }
}

pub trait DrawingsApi {
    fn get_translator_formats2(&self, did: &str, wid: &str, eid: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn translate_format4(&self, did: &str, wv: &str, wvid: &str, eid: &str, bt_translate_format_params: crate::models::BtTranslateFormatParams) -> Box<dyn Future<Item = crate::models::BtTranslationRequestInfo, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>DrawingsApi for DrawingsApiClient<C> {
    fn get_translator_formats2(&self, did: &str, wid: &str, eid: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/drawings/d/{did}/w/{wid}/e/{eid}/translationformats".to_string())
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wid".to_string(), wid.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn translate_format4(&self, did: &str, wv: &str, wvid: &str, eid: &str, bt_translate_format_params: crate::models::BtTranslateFormatParams) -> Box<dyn Future<Item = crate::models::BtTranslationRequestInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/drawings/d/{did}/{wv}/{wvid}/e/{eid}/translations".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wv".to_string(), wv.to_string());
        req = req.with_path_param("wvid".to_string(), wvid.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_body_param(bt_translate_format_params);

        req.execute(self.configuration.borrow())
    }

}
