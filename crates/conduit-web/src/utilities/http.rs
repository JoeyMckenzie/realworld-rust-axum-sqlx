use js_sys::JSON;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Performs an HTTP request asynchnonously by given URL
/// and returns parsed JSON.
pub async fn http_request_json(url: &str, method: &str, body: Option<JsValue>) -> Result<JsValue, JsValue> {
    // prepare request opts
    let mut request_options = RequestInit::new();
    request_options.method(method);
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

    // get JSON data
    let json = if response_meta.status() == 200 {
        JsFuture::from(response_meta.json()?).await?
    } else {
        JsValue::null()
    };

    Ok(json)
}
