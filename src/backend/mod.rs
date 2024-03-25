use std::collections::HashMap;

use anyhow::{anyhow, Ok};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::code_context::{Code, CodeParameter};

use self::fetch::{do_get_json, ResourceData};

mod fetch;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NodeInfo {
    pub node: Node,
    pub children: Vec<NodeInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    let url = "http://172.16.141.158:9999/magic/web/wdResource";
    let res = do_get_json(url, None).await?;
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
    let url = format!(
        "http://172.16.141.158:9999/magic/web/resource/file/wdhis:{}",
        id
    );
    let res = do_get_json::<Code>(&url, None).await?;
    if res.code != 1 {
        return Err(anyhow::Error::msg(res.message));
    }
    Ok(res.data)
}

pub async fn send_code(code: Code, mut url: String) -> anyhow::Result<String> {
    let method = code.method.as_ref().expect("code method is None");
    match method.as_str() {
        "GET" => {
            url.push_str(&generate_get_params(&code.parameters));
            let result = do_get_json::<Value>(&url, None).await?;
            let result = serde_json::to_string(&result)?;
            return Ok(result);
        }
        "POST" => {}
        _ => {
            return Err(anyhow!(format!("unsupport http method: {}", &method)));
        }
    };
    Ok("".to_string())
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
