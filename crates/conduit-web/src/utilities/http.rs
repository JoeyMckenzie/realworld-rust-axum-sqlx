use js_sys::JSON;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug)]
enum Method {
    Get,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub async fn get<T>(url: &str) -> Result<T, JsValue>
where
    T: Default + for<'a> serde::de::Deserialize<'a>,
{
    with_request::<T>(url, Method::Get, None).await
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
    let request = Request::new_with_str_and_init(url, &request_options)?;
    request.headers().set("Accept", "application/json")?;

    // set Content-Type to application/json
    if body.is_some() {
        request.headers().set("Content-Type", "application/json")?;
    }

    // make request
    let window = web_sys::window().expect("window was not found");
    let http_response = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response_meta: Response = http_response.dyn_into().unwrap();

    if response_meta.status() == 200 {
        let json_content = JsFuture::from(response_meta.json()?).await?;
        let struct_response: T = json_content.into_serde().unwrap();
        return Ok(struct_response);
    }

    Ok(T::default())
}
