use js_sys::Reflect;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub struct JsError {
    pub msg: String,
}

impl From<JsValue> for JsError {
    fn from(value: JsValue) -> Self {
        if let Some(msg) = value.as_string() {
            JsError { msg }
        } else {
            JsError {
                msg: "js 函数调用出错".to_string(),
            }
        }
    }
}

pub fn set_value(obj: &JsValue, key: &str, value: &str) -> Result<(), JsError> {
    Reflect::set(obj, &JsValue::from_str(key), &JsValue::from_str(value))?;
    Ok(())
}

pub fn get_str_value(obj: &JsValue, key: &str) -> Result<String, JsError> {
    let res = Reflect::get(&obj, &JsValue::from_str(key))?;
    if res.is_string() {
        return Ok(res.as_string().unwrap());
    }
    Err(JsError {
        msg: "get failed no string value".to_string(),
    })
}
