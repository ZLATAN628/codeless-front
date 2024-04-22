use std::collections::HashMap;

use anyhow::anyhow;
use gloo::{net::http::Headers, utils::window};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::code_context::{Code, CodeParameter};

use self::fetch::{do_get_json, do_get_json_with_headers, do_post_json, ResourceData};

mod fetch;

lazy_static! {
    pub static ref ORIGIN: String = window().location().origin().unwrap();
    pub static ref API_ORIGIN: String = String::from("http://172.16.141.158:9999/");
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NodeInfo {
    pub node: Node,
    pub children: Vec<NodeInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Node {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<u64>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<u64>,
    #[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[serde(rename = "updateBy")]
    pub update_by: Option<String>,
    #[serde(default)]
    pub paths: Vec<String>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub properties: HashMap<String, Value>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
}

/**
 * 获取所有资源
 */
pub async fn get_wd_resource() -> anyhow::Result<ResourceData> {
    // TODO: url
    let url = format!("{}magic/web/wdResource", *API_ORIGIN);
    let res = do_get_json(&url, None).await?;
    if res.code != 1 {
        return Err(anyhow::Error::msg(res.message));
    }
    Ok(res.data)
}

pub async fn get_api_info() -> anyhow::Result<NodeInfo> {
    let result = get_wd_resource().await?;
    if let Some(api_info) = result.api {
        Ok(api_info)
    } else {
        Err(anyhow::Error::msg("api_info is None"))
    }
}

pub async fn get_code_by_id(id: &str) -> anyhow::Result<Code> {
    let url = format!("{}magic/web/resource/file/wdhis:{}", *API_ORIGIN, id);
    let res = do_get_json::<Code>(&url, None).await?;
    if res.code != 1 {
        return Err(anyhow::Error::msg(res.message));
    }
    Ok(res.data)
}

pub async fn send_code(code: Code, mut url: String) -> (anyhow::Result<String>, Option<Headers>) {
    let method = code.method.as_ref().expect("code method is None");
    match method.as_str() {
        "GET" => {
            url.push_str(&generate_get_params(&code.parameters));
            let mut headers = HashMap::new();
            headers.insert(
                "Magic-Request-Client-Id".to_string(),
                "f32121313414".to_string(),
            );
            headers.insert("Magic-Request-Script-Id".to_string(), code.id.clone());

            let (result, headers) = do_get_json_with_headers::<Value>(&url, Some(&headers)).await;
            let result = match result {
                Ok(result) => match serde_json::to_string(&result) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e),
            };
            return (result, headers);
        }
        "POST" => {}
        _ => {
            return (
                Err(anyhow!(format!("unsupport http method: {}", &method))),
                None,
            );
        }
    };
    (Ok("".to_string()), None)
}

pub async fn save_code(code: Code) -> anyhow::Result<String> {
    let url = format!("{}magic/web/wdResource/file/api/save", *API_ORIGIN);
    let param = serde_json::to_string(&code)?;
    let data = do_post_json::<String, String>(&url, param, None).await?;
    // code id
    Ok(data.data)
}

fn generate_get_params(params: &Vec<CodeParameter>) -> String {
    let params = params
        .iter()
        .map(|e| {
            format!(
                "{}={}",
                e.name.as_ref().unwrap_or(&"".to_string()),
                e.value.as_ref().unwrap_or(&"".to_string())
            )
        })
        .collect::<Vec<String>>()
        .join("&");
    format!("?{}", params)
}
