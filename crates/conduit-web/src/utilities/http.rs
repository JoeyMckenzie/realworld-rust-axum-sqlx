use gloo::net::http::Method;
use js_sys::JSON;
use lazy_static::lazy_static;
use log::info;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::storage::get_token;

lazy_static! {
    static ref API_BASE_URL: &'static str = "https://api.realworld.io/api";
}

pub async fn get<T>(url: &str) -> Result<T, JsValue>
where
    T: Default + for<'a> serde::de::Deserialize<'a>,
{
    with_request::<T>(url, Method::GET, None).await
}

pub async fn post<T, K>(url: &str, body: K) -> Result<T, JsValue>
where
    T: Default + for<'a> serde::de::Deserialize<'a>,
    K: Serialize,
{
    let request_body = JsValue::from_serde(&body).unwrap();
    with_request::<T>(url, Method::POST, Some(request_body)).await
}

pub async fn put<T, K>(url: &str, body: K) -> Result<T, JsValue>
where
    T: Default + for<'a> serde::de::Deserialize<'a>,
    K: Serialize,
{
    let request_body = JsValue::from_serde(&body).unwrap();
    with_request::<T>(url, Method::PUT, Some(request_body)).await
}

/// Performs an HTTP request asynchnonously by given URL
/// and returns parsed JSON.
async fn with_request<T>(url: &str, method: Method, body: Option<JsValue>) -> Result<T, JsValue>
where
    T: Default + for<'a> serde::de::Deserialize<'a>,
{
    // prepare request options
    let mut request_options = RequestInit::new();
    request_options.method(&method.to_string());
    request_options.mode(RequestMode::Cors);

    // set JSON body
    if body.is_some() {
        request_options.body(Some(&JSON::stringify(body.as_ref().unwrap()).unwrap()));
    }

    // prepare request
    let request = Request::new_with_str_and_init(&format!("{}{}", *API_BASE_URL, url), &request_options)?;
    request.headers().set("Accept", "application/json")?;

    if let Ok(token) = get_token() {
        request.headers().set("Authorization", &format!("Token {}", token))?;
    }

    // set Content-Type to application/json
    if body.is_some() {
        request.headers().set("Content-Type", "application/json")?;
    }

    // make request
    let window = web_sys::window().expect("window was not found");
    let http_response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response_meta: Response = http_response.dyn_into().unwrap();
    let json_content = JsFuture::from(response_meta.json()?).await?;

    if response_meta.status() == 200 {
        info!("{:?}", response_meta);
        let as_struct_response: T = json_content.into_serde().unwrap();
        return Ok(as_struct_response);
    }

    Err(json_content)
}
