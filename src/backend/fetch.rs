use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use wasm_bindgen::JsValue;

use gloo::{
    console::log,
    net::http::{Headers, Request, Response},
};

use super::NodeInfo;

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ResourceData {
    pub datasource: Option<NodeInfo>,
    pub api: Option<NodeInfo>,
}

pub async fn do_get(
    url: &str,
    headers: Option<&HashMap<String, String>>,
) -> anyhow::Result<Response> {
    let mut req = Request::get(url);
    if let Some(headers) = headers {
        for (k, v) in headers.iter() {
            req = req.header(k, v);
        }
    }
    let res = req.send().await.unwrap();
    Ok(res)
}

pub async fn do_get_json<T>(
    url: &str,
    headers: Option<&HashMap<String, String>>,
) -> anyhow::Result<GenericResponse<T>>
where
    T: DeserializeOwned,
{
    let js_result = do_get(url, headers).await?;
    match js_result.json::<GenericResponse<T>>().await {
        Ok(result) => Ok(result),
        Err(e) => {
            log!(format!("{:?}", e));
            Err(e.into())
        }
    }
}

pub async fn do_get_json_with_headers<T>(
    url: &str,
    headers: Option<&HashMap<String, String>>,
) -> (anyhow::Result<GenericResponse<T>>, Option<Headers>)
where
    T: DeserializeOwned,
{
    let resp = match do_get(url, headers).await {
        Ok(resp) => resp,
        Err(e) => {
            return (Err(e.into()), None);
        }
    };
    let result = match resp.json::<GenericResponse<T>>().await {
        Ok(result) => Ok(result),
        Err(e) => Err(e.into()),
    };

    (result, Some(resp.headers()))
}

pub async fn do_post<F>(
    url: &str,
    body: F,
    headers: Option<&HashMap<String, String>>,
) -> anyhow::Result<Response>
where
    F: Into<JsValue>,
{
    let mut req = Request::post(url);
    if let Some(headers) = headers {
        for (k, v) in headers.iter() {
            req = req.header(k, v);
        }
    }
    let res = req.body(body)?.send().await?;
    Ok(res)
}

pub async fn do_post_json<T, F>(
    url: &str,
    body: F,
    headers: Option<&HashMap<String, String>>,
) -> anyhow::Result<GenericResponse<T>>
where
    T: DeserializeOwned,
    F: Into<JsValue>,
{
    let js_result = do_post(url, body, headers).await?;
    match js_result.json::<GenericResponse<T>>().await {
        Ok(result) => Ok(result),
        Err(e) => {
            log!(format!("{:?}", e));
            Err(e.into())
        }
    }
}
