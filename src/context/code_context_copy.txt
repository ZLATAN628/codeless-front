use std::{cell::RefCell, collections::HashMap, rc::Rc};

use gloo::storage::{LocalStorage, Storage};
use monaco::api::TextModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yew::{prelude::*, virtual_dom::VNode};

use crate::{backend::NodeInfo, components::editor::monaco_editor::editor::MonacoEditor};

#[derive(Debug, PartialEq, Clone)]
pub struct CodeTab(String, bool);

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

#[derive(Debug, PartialEq, Clone)]
pub struct CodeTabs {
    pub data: HashMap<String, CodeTab>,
    pub order: Vec<String>,
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

#[derive(Debug, PartialEq, Clone)]
pub struct GroupNode {
    path: String,
    parent_id: String,
}

#[derive(Debug, Clone)]
pub struct CodesContext {
    code: UseStateHandle<Code>,
    code_tabs: UseStateHandle<Rc<RefCell<CodeTabs>>>,
    code_editors: UseStateHandle<HashMap<String, VNode>>,
    group_map: UseStateHandle<HashMap<String, GroupNode>>,
}

impl CodesContext {
    pub fn new(
        code: UseStateHandle<Code>,
        code_tabs: UseStateHandle<Rc<RefCell<CodeTabs>>>,
        code_editors: UseStateHandle<HashMap<String, VNode>>,
        group_map: UseStateHandle<HashMap<String, GroupNode>>,
    ) -> Self {
        Self {
            code,
            code_tabs,
            code_editors,
            group_map,
        }
    }

    pub fn put_code(&self, code: Code, save: bool) {
        if save {
            let key = &code.id;
            let value = serde_json::to_string(&code).unwrap();
            LocalStorage::set(key, value).unwrap();
            // 创建代码编辑器实例
            self.generate_code_editor(key, &code.script());
        }
        self.put_code_tabs(&code.id, &code.name.as_ref().unwrap_or(&"".to_string()));
        self.code.set(code);
    }

    pub fn put_code_tabs(&self, id: &str, name: &str) {
        let mut tabs = self.code_tabs.borrow_mut();
        tabs.data
            .insert(id.to_string(), CodeTab(name.to_string(), false));
        let id = id.to_string();
        if !tabs.order.contains(&id) {
            tabs.order.push(id);
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

    pub fn current_code_tabs(&self) -> Rc<RefCell<CodeTabs>> {
        (*self.code_tabs).clone()
    }

    pub fn code_editors(&self) -> Option<&VNode> {
        self.code_editors.get(self.code.id())
    }

    pub fn set_current_code_id(&self, id: String) {
        if let Some(value) = self.load_code(&id) {
            self.code.set(value)
        }
    }

    pub fn did_change_editor(&self, id: String) {
        let mut code_tabs = self.code_tabs.borrow_mut();
        if let Some(tab) = code_tabs.get_tab_mut(&id) {
            tab.set_updated_state(true);
            // TODO: 暂时没找到好的办法 去触发重新渲染
            let mut code = (*self.code).clone();
            code.yew_state += 1;
            self.code.set(code);
        }
    }

    pub fn clear_storage(&self) {
        LocalStorage::clear();
    }

    pub fn is_update_code(&self, id: &str) -> bool {
        if let Some(tab) = self.code_tabs.borrow().get_tab(id) {
            tab.is_update()
        } else {
            false
        }
    }

    pub fn remove_code(&self, id: &str) {
        let mut tabs = self.code_tabs.borrow_mut();
        tabs.remove_tab(id);
        LocalStorage::delete(id);
    }

    pub fn set_next_code(&self, pre_id: &str) {
        if pre_id == self.code.id() {
            let tabs = self.code_tabs.borrow();
            if let Some(id) = tabs.get_first_id() {
                if let Some(code) = self.load_code(&id) {
                    self.code.set(code);
                    return;
                }
            }
            self.code.set(Code::default());
        } else {
            // TODO: 暂时没找到好的办法 去触发重新渲染
            let mut code = (*self.code).clone();
            code.yew_state += 1;
            self.code.set(code);
        }
    }

    fn generate_code_editor(&self, key: &str, script: &str) {
        let text_model = TextModel::create(script, Some("sql"), None).unwrap();
        let code_editor = html! {
            // key 一定要加 不然会复用 先前创建的组件
            <MonacoEditor text_model={text_model.clone()} id={key.to_string()} key={key.to_string()} />
        };
        let mut new_code_editor = (*self.code_editors).clone();
        new_code_editor.insert(key.to_string(), code_editor);
        self.code_editors.set(new_code_editor);
    }

    pub fn save_group_info(&self, node: &NodeInfo) {
        let mut group_map = (*self.group_map).clone();
        // 第一层 root 直接调 children
        for n in node.children.iter() {
            self.extract_group_info(n, &mut group_map);
        }
        self.group_map.set(group_map);
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

    pub fn update_response(&self, result: String) {
        let mut code = (*self.code).clone();
        code.response_body = Some(result);
        self.code.set(code);
    }
}

impl PartialEq for CodesContext {
    fn eq(&self, other: &Self) -> bool {
        *self.code == *other.code
    }
}

#[derive(PartialEq, Properties)]
pub struct CodesProviderProps {
    pub children: Children,
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
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
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
    let code = Code::default();
    let code = use_state(|| code);
    let code_tabs = use_state(|| {
        Rc::new(RefCell::new(CodeTabs {
            data: HashMap::new(),
            order: Vec::new(),
        }))
    });
    let code_editors = use_state(|| HashMap::new());
    let group_map = use_state(|| HashMap::new());
    let context = CodesContext::new(code, code_tabs, code_editors, group_map);
    html! {
        <ContextProvider<CodesContext> context={context}>
            {props.children.clone()}
        </ContextProvider<CodesContext>>
    }
}
