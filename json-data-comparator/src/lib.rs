mod utils;
use itertools::Itertools;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::VecDeque;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct JsonFormatter {
    #[serde(flatten)]
    input_data: Value,
}

#[wasm_bindgen]
pub struct SearchResults {
    // line_number: i32,
    line_result: String,
}

impl fmt::Display for SearchResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.line_result)
    }
}
/// cd www/
/// npm run init
/// wasm-pack build
#[wasm_bindgen]
impl JsonFormatter {
    pub fn display_full_json(&self) -> String {
        serde_json::to_string_pretty(&self.input_data).unwrap()
    }

    pub fn search_attributes(&self, search_str: &str) -> Result<String, String> {
        let mut attr_list: VecDeque<&str> = search_str.split('.').collect();
        let reference_attr: Option<&&str> = attr_list.front();
        match reference_attr {
            None => return Err(String::from("Input should not be empty")),
            // move on if there's something to work with
            Some(_) => (),
        }

        let mut lookup: &Value = &self.input_data;
        while attr_list.len() > 0 {
            let key = attr_list.pop_front();
            let current = match key {
                None => break,
                Some(x) => x,
            };
            let value = &lookup[current.to_string()];

            match value {
                // IF THE CURRENT VALUE IS AN ARRAY, USE THE KEY POPPED TO ITERATE THRU FOR KEY IN OBJECTS
                Value::Array(arr) => {
                    let get_attr_search = match attr_list.pop_front() {
                        None => {
                            return Err(String::from(format!(
                                "Expecting an attribute lookup after input search '{current}'"
                            )))
                        }
                        Some(x) => x,
                    };
                    return Ok(arr
                        .into_iter()
                        .map(|x| x[get_attr_search].to_string())
                        .join("\r\n"));
                }
                // IF ITS AN OBJECT, THEN USE THE KEY ATTRIBUTE ACCESS
                Value::Object(_) => {
                    lookup = value;
                }
                _ => return Ok(value.to_string()),
            }
        }
        return Ok(String::from(""));
    }
}

#[wasm_bindgen]
pub fn init_formatter(data: &str) -> Result<JsonFormatter, String> {
    let parse = match serde_json::from_str(data) {
        Ok(parse) => JsonFormatter { input_data: parse },
        Err(_) => return Err(String::from("not in json format")),
    };
    Ok(parse)
}
