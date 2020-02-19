/*
 * Onshape REST API
 *
 * The Onshape REST API consumed by all clients.
 *
 * The version of the OpenAPI document: 1.104
 * Contact: api-support@onshape.zendesk.com
 * Generated by: https://openapi-generator.tech
 */

use hyper;

pub struct Configuration<C: hyper::client::Connect> {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: hyper::client::Client<C>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl<C: hyper::client::Connect> Configuration<C> {
    pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
        Configuration {
            base_path: "https://cad.onshape.com".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.104/rust".to_owned()),
            client: client,
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}
