use std::{collections::HashMap, rc::Rc};

use gloo::{
    net::http::Headers,
    storage::{LocalStorage, Storage},
};
use monaco::api::TextModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::{prelude::*, virtual_dom::VNode};

use crate::{backend::NodeInfo, components::editor::monaco_editor::editor::MonacoEditor};

pub type CodesContext = UseReducerHandle<CodesState>;

#[derive(Debug, PartialEq, Clone)]
pub struct CodeTab(String, bool);

#[derive(Debug, PartialEq, Clone)]
pub struct CodeTabs {
    pub data: HashMap<String, CodeTab>,
    pub order: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct GroupNode {
    path: String,
    parent_id: String,
}

#[derive(Clone, Properties)]
pub struct CodesState {
    code: Code,
    code_tabs: CodeTabs,
    code_editors: HashMap<String, VNode>,
    group_map: HashMap<String, GroupNode>,
    need_update: u64,
}

#[derive(Debug)]
pub enum CodesStateMsg {
    UpdateCode(Code, bool),
    RemoveCode(String),
    #[allow(dead_code)]
    UpdateCodeTab(String, String),
    ChangeCode(String),
    EditorChanged(String),
    SetNextCode(String),
    SaveGroup(NodeInfo),
    UpdateResp(String, Option<Headers>),
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Default)]
pub struct CodeParameter {
    pub name: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
    pub required: bool,
    pub data_type: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
    #[serde(rename = "validateType")]
    pub validate_type: Option<String>,
    pub error: Option<String>,
    pub expression: Option<String>,
    // children: Option<Vec<CodeParameter>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Code {
    pub id: String,
    pub name: Option<String>,
    pub path: Option<String>,
    #[serde(rename = "createTime")]
    pub create_time: Option<u64>,
    #[serde(rename = "updateTime")]
    pub update_time: Option<u64>,
    #[serde(rename = "createBy")]
    pub create_by: Option<String>,
    #[serde(rename = "updateBy")]
    pub update_by: Option<String>,
    pub paths: Vec<String>,
    pub options: Vec<String>,
    pub properties: HashMap<String, Value>,
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    pub script: Option<String>,
    pub method: Option<String>,
    #[serde(default)]
    pub yew_state: u64,
    #[serde(default)]
    pub parameters: Vec<CodeParameter>,
    #[serde(rename = "responseBody")]
    pub response_body: Option<String>,
    #[serde(default)]
    pub resp_headers: HashMap<String, String>,
}

#[derive(PartialEq, Properties)]
pub struct CodesProviderProps {
    pub children: Children,
}

impl CodeTab {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn is_update(&self) -> bool {
        self.1
    }

    pub fn set_updated_state(&mut self, state: bool) {
        self.1 = state;
    }
}

impl CodeTabs {
    pub fn get_tab(&self, id: &str) -> Option<&CodeTab> {
        self.data.get(id)
    }

    pub fn get_tab_mut(&mut self, id: &str) -> Option<&mut CodeTab> {
        self.data.get_mut(id)
    }

    pub fn remove_tab(&mut self, id: &str) {
        self.data.remove(id);
        self.order.retain(|x| x != id);
    }

    pub fn get_first_id(&self) -> Option<String> {
        if self.order.is_empty() {
            None
        } else {
            Some(self.order[0].clone())
        }
    }
}

impl PartialEq for CodesState {
    fn eq(&self, other: &Self) -> bool {
        self.need_update == other.need_update
    }
}

impl Reducible for CodesState {
    type Action = CodesStateMsg;
    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let ns = Rc::make_mut(&mut self);
        match action {
            CodesStateMsg::UpdateCode(code, save) => {
                if save {
                    let key = &code.id;
                    let value = serde_json::to_string(&code).unwrap();
                    LocalStorage::set(key, value).unwrap();
                    // 创建代码编辑器实例
                    ns.generate_code_editor(key, &code.script());
                }
                ns.put_code_tabs(&code.id, &code.name.as_ref().unwrap_or(&"".to_string()));
                let ns = Rc::make_mut(&mut self);
                ns.code = code;
                ns.need_update += 1;
            }
            CodesStateMsg::RemoveCode(id) => {
                ns.code_tabs.remove_tab(&id);
                LocalStorage::delete(&id);
            }
            CodesStateMsg::UpdateCodeTab(id, name) => {
                ns.put_code_tabs(&id, &name);
            }
            CodesStateMsg::ChangeCode(id) => {
                if let Some(code) = ns.load_code(&id) {
                    ns.code = code;
                    ns.need_update += 1;
                }
            }
            CodesStateMsg::EditorChanged(id) => {
                if let Some(tab) = ns.code_tabs.get_tab_mut(&id) {
                    tab.set_updated_state(true);
                    ns.need_update += 1;
                }
            }
            CodesStateMsg::SetNextCode(pre_id) => {
                ns.need_update += 1;
                if pre_id == ns.code.id() {
                    if let Some(id) = ns.code_tabs.get_first_id() {
                        if let Some(code) = ns.load_code(&id) {
                            ns.code = code;
                            return self;
                        }
                    }
                    ns.code = Code::default();
                }
            }
            CodesStateMsg::SaveGroup(node) => {
                let mut group_map = HashMap::new();
                // 第一层 root 直接调 children
                for n in node.children.iter() {
                    ns.extract_group_info(n, &mut group_map);
                }
                ns.group_map = group_map;
            }
            CodesStateMsg::UpdateResp(result, headers) => {
                if let Some(headers) = headers {
                    let map = &mut ns.code.resp_headers;
                    map.clear();
                    for (k, v) in headers.entries() {
                        map.insert(k, v);
                    }
                }
                ns.need_update += 1;
                ns.code.response_body = Some(result);
            }
        };
        self
    }
}

impl CodesState {
    pub fn new() -> Self {
        Self {
            code: Code::default(),
            code_tabs: CodeTabs {
                data: HashMap::new(),
                order: Vec::new(),
            },
            code_editors: HashMap::new(),
            group_map: HashMap::new(),
            need_update: 0,
        }
    }

    pub fn put_code_tabs(&mut self, id: &str, name: &str) {
        self.code_tabs
            .data
            .insert(id.to_string(), CodeTab(name.to_string(), false));
        let id = id.to_string();
        if !self.code_tabs.order.contains(&id) {
            self.code_tabs.order.push(id);
        }
    }

    pub fn load_code(&self, id: &str) -> Option<Code> {
        match LocalStorage::get::<String>(id) {
            Ok(value) => {
                let code: Code = serde_json::from_str(&value).unwrap();
                Some(code)
            }
            Err(_) => None,
        }
    }

    pub fn current_code(&self) -> &Code {
        &self.code
    }

    pub fn current_code_tabs(&self) -> &CodeTabs {
        &self.code_tabs
    }

    pub fn code_editors(&self) -> Option<&VNode> {
        self.code_editors.get(self.code.id())
    }

    pub fn clear_storage(&self) {
        LocalStorage::clear();
    }

    pub fn is_update_code(&self, id: &str) -> bool {
        if let Some(tab) = self.code_tabs.get_tab(id) {
            tab.is_update()
        } else {
            false
        }
    }

    fn generate_code_editor(&mut self, key: &str, script: &str) {
        let text_model = TextModel::create(script, Some("sql"), None).unwrap();
        let code_editor = html! {
            // key 一定要加 不然会复用 先前创建的组件
            <MonacoEditor text_model={text_model.clone()} id={key.to_string()} key={key.to_string()} />
        };
        self.code_editors.insert(key.to_string(), code_editor);
    }

    fn extract_group_info(&self, node: &NodeInfo, map: &mut HashMap<String, GroupNode>) {
        if node.node.group_id.is_none() {
            // is group
            map.insert(
                node.node.id.clone().expect("group id is none"),
                GroupNode {
                    path: node
                        .node
                        .path
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .to_string(),
                    parent_id: node.node.parent_id.clone().unwrap(),
                },
            );
        }

        for n in node.children.iter() {
            self.extract_group_info(n, map);
        }
    }

    pub fn get_final_path(&self, path: &str, group_id: &str) -> String {
        let mut urls = vec![path];
        let mut group_id = group_id.to_string();
        while let Some(gnode) = self.group_map.get(&group_id) {
            urls.push(&gnode.path);
            group_id = gnode.parent_id.to_string()
        }
        let url: String = urls.into_iter().rev().collect::<Vec<&str>>().concat();
        // TODO: HOST
        format!("http://172.16.141.158:9999{}", url)
    }
}

impl Code {
    pub fn script(&self) -> String {
        let empty = "".to_string();
        self.script.as_ref().unwrap_or_else(|| &empty).to_string()
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[hook]
pub(crate) fn use_codes() -> CodesContext {
    use_context::<CodesContext>().unwrap()
}

#[function_component(CodesProvider)]
pub fn codes_provider(props: &CodesProviderProps) -> Html {
    let context = use_reducer(|| CodesState::new());
    html! {
        <ContextProvider<CodesContext> context={context}>
            {props.children.clone()}
        </ContextProvider<CodesContext>>
    }
}
