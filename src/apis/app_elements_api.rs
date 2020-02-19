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

pub struct AppElementsApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> AppElementsApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> AppElementsApiClient<C> {
        AppElementsApiClient {
            configuration,
        }
    }
}

pub trait AppElementsApi {
    fn commit_transaction(&self, did: &str, eid: &str, wid: &str, tid: &str, bt_app_element_commit_transaction_params: crate::models::BtAppElementCommitTransactionParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn create4(&self, did: &str, wid: &str, bt_app_element_params: crate::models::BtAppElementParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn create_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, bt_app_element_reference_params: crate::models::BtAppElementReferenceParams) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>>;
    fn delete_content(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, sid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, description: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn delete_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, description: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>>;
    fn get_history(&self, did: &str, eid: &str, wvm: &str, wvmid: &str) -> Box<dyn Future<Item = crate::models::BtAppElementHistoryInfo, Error = Error<serde_json::Value>>>;
    fn get_sub_element_content(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, change_id: Option<&str>, base_change_id: Option<&str>, subelement_id: Option<&str>, link_document_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementContentInfo, Error = Error<serde_json::Value>>>;
    fn get_subelement_ids(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, change_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn resolve_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, include_internal: Option<bool>, link_document_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceResolveInfo, Error = Error<serde_json::Value>>>;
    fn resolve_references(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, include_internal: Option<bool>, link_document_id: Option<&str>, reference_ids: Option<&str>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
    fn start_transaction(&self, did: &str, eid: &str, wid: &str, bt_app_element_start_transaction_params: crate::models::BtAppElementStartTransactionParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn update4(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, bt_app_element_update_params: crate::models::BtAppElementUpdateParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>>;
    fn update_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, bt_app_element_reference_params: crate::models::BtAppElementReferenceParams) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>>;
}

impl<C: hyper::client::Connect>AppElementsApi for AppElementsApiClient<C> {
    fn commit_transaction(&self, did: &str, eid: &str, wid: &str, tid: &str, bt_app_element_commit_transaction_params: crate::models::BtAppElementCommitTransactionParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/w/{wid}/e/{eid}/transactions/{tid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wid".to_string(), wid.to_string());
        req = req.with_path_param("tid".to_string(), tid.to_string());
        req = req.with_body_param(bt_app_element_commit_transaction_params);

        req.execute(self.configuration.borrow())
    }

    fn create4(&self, did: &str, wid: &str, bt_app_element_params: crate::models::BtAppElementParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/w/{wid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("wid".to_string(), wid.to_string());
        req = req.with_body_param(bt_app_element_params);

        req.execute(self.configuration.borrow())
    }

    fn create_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, bt_app_element_reference_params: crate::models::BtAppElementReferenceParams) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_body_param(bt_app_element_reference_params);

        req.execute(self.configuration.borrow())
    }

    fn delete_content(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, sid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, description: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/subelements/{sid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = parent_change_id {
            req = req.with_query_param("parentChangeId".to_string(), s.to_string());
        }
        if let Some(ref s) = description {
            req = req.with_query_param("description".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("sid".to_string(), sid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn delete_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, description: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Delete, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = parent_change_id {
            req = req.with_query_param("parentChangeId".to_string(), s.to_string());
        }
        if let Some(ref s) = description {
            req = req.with_query_param("description".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("rid".to_string(), rid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_history(&self, did: &str, eid: &str, wvm: &str, wvmid: &str) -> Box<dyn Future<Item = crate::models::BtAppElementHistoryInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/history".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_sub_element_content(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, change_id: Option<&str>, base_change_id: Option<&str>, subelement_id: Option<&str>, link_document_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementContentInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = change_id {
            req = req.with_query_param("changeId".to_string(), s.to_string());
        }
        if let Some(ref s) = base_change_id {
            req = req.with_query_param("baseChangeId".to_string(), s.to_string());
        }
        if let Some(ref s) = subelement_id {
            req = req.with_query_param("subelementId".to_string(), s.to_string());
        }
        if let Some(ref s) = link_document_id {
            req = req.with_query_param("linkDocumentId".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn get_subelement_ids(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, change_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content/ids".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = change_id {
            req = req.with_query_param("changeId".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn resolve_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, include_internal: Option<bool>, link_document_id: Option<&str>) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceResolveInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = parent_change_id {
            req = req.with_query_param("parentChangeId".to_string(), s.to_string());
        }
        if let Some(ref s) = include_internal {
            req = req.with_query_param("includeInternal".to_string(), s.to_string());
        }
        if let Some(ref s) = link_document_id {
            req = req.with_query_param("linkDocumentId".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("rid".to_string(), rid.to_string());

        req.execute(self.configuration.borrow())
    }

    fn resolve_references(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, transaction_id: Option<&str>, parent_change_id: Option<&str>, include_internal: Option<bool>, link_document_id: Option<&str>, reference_ids: Option<&str>) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Get, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/resolvereferences".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        if let Some(ref s) = transaction_id {
            req = req.with_query_param("transactionId".to_string(), s.to_string());
        }
        if let Some(ref s) = parent_change_id {
            req = req.with_query_param("parentChangeId".to_string(), s.to_string());
        }
        if let Some(ref s) = include_internal {
            req = req.with_query_param("includeInternal".to_string(), s.to_string());
        }
        if let Some(ref s) = link_document_id {
            req = req.with_query_param("linkDocumentId".to_string(), s.to_string());
        }
        if let Some(ref s) = reference_ids {
            req = req.with_query_param("referenceIds".to_string(), s.to_string());
        }
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    fn start_transaction(&self, did: &str, eid: &str, wid: &str, bt_app_element_start_transaction_params: crate::models::BtAppElementStartTransactionParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/w/{wid}/e/{eid}/transactions".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wid".to_string(), wid.to_string());
        req = req.with_body_param(bt_app_element_start_transaction_params);

        req.execute(self.configuration.borrow())
    }

    fn update4(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, bt_app_element_update_params: crate::models::BtAppElementUpdateParams) -> Box<dyn Future<Item = crate::models::BtAppElementModifyInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/content".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_body_param(bt_app_element_update_params);

        req.execute(self.configuration.borrow())
    }

    fn update_reference(&self, did: &str, eid: &str, wvm: &str, wvmid: &str, rid: &str, bt_app_element_reference_params: crate::models::BtAppElementReferenceParams) -> Box<dyn Future<Item = crate::models::BtAppElementReferenceInfo, Error = Error<serde_json::Value>>> {
        let mut req = __internal_request::Request::new(hyper::Method::Post, "/api/appelements/d/{did}/{wvm}/{wvmid}/e/{eid}/references/{rid}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("did".to_string(), did.to_string());
        req = req.with_path_param("eid".to_string(), eid.to_string());
        req = req.with_path_param("wvm".to_string(), wvm.to_string());
        req = req.with_path_param("wvmid".to_string(), wvmid.to_string());
        req = req.with_path_param("rid".to_string(), rid.to_string());
        req = req.with_body_param(bt_app_element_reference_params);

        req.execute(self.configuration.borrow())
    }

}
