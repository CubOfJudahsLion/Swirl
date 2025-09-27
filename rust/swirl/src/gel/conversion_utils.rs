//! Module  :   JS->WASM conversion utilities
//! Purpose :   Allows us to check JS types into Rust types.
//! Version :   0.1.0
//! Author  :   Alex Feterman
//! Created :   Thu 02/17/2022 02:44 PM
//! Modified:   Tue 04/19/2022 08:58 AM


use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Array, JsString};


// /// Attempts to convert a `JsValue` to a `String`.
// pub fn js_as_string(js_val: &JsValue, func_name: &str, param_name: &str) -> Result<String, JsValue> {
//     js_val
//         .as_string()
//         .ok_or_else(|| JsValue::from_str(&format!(
//             "{}: Error: param '{}' is not a string.",
//             func_name,
//             param_name
//         )))
// }
// 
// /// Attempts to convert a `JsValue` to an `f64`.
// pub fn js_as_f64(js_val: &JsValue, func_name: &str, param_name: &str) -> Result<f64, JsValue> {
//     js_val
//         .as_f64()
//         .ok_or_else(|| JsValue::from_str(&format!(
//             "{}: Error: param '{}' is not a number.",
//             func_name,
//             param_name
//         )))
// }
// 
// /// Attempts to convert a `JsValue` to a `bool`.
// pub fn js_as_bool(js_val: &JsValue, func_name: &str, param_name: &str) -> Result<bool, JsValue> {
//     js_val
//         .as_bool()
//         .ok_or_else(|| JsValue::from_str(&format!(
//             "{}: Error: param '{}' is not a boolean.",
//             func_name,
//             param_name
//         )))
// }

/// Attempts to convert a `JsValue` to a `Box<[String]>`.
pub fn js_as_string_slice_box(js_val: JsValue, func_name: &str, param_name: &str) -> Result<Box<[String]>, JsValue> {
    let mut in_vec  = js_val.dyn_ref::<Array>()
        .ok_or_else(|| JsValue::from_str(&format!(
            "{}: Error: param '{}' is not an array.",
            func_name,
            param_name
        )))?
        .to_vec();
    let mut out_vec = Vec::<String>::with_capacity(in_vec.len());
    for (idx, reel_str) in in_vec.drain(..).enumerate() {
        out_vec.push(
            reel_str
                .as_string()
                .ok_or_else(|| JsValue::from_str(&format!(
                    "{}: Error: array param '{}' contains non-string value '{}' at index #{}.",
                    func_name,
                    param_name,
                    JsString::from(reel_str),
                    idx
                )))?
        );
    }
    Ok(out_vec.into_boxed_slice())
}

/// Attempts to convert a `JsValue` to an `Option<Box<[String]>>`.
pub fn js_as_opt_string_slice_box(js_value: JsValue, func_name: &str, param_name: &str) -> Result<Option<Box<[String]>>, JsValue> {
    if js_value.is_null() {
        Ok(None)
    }
    else {
        Ok(Some(js_as_string_slice_box(js_value, func_name, param_name)?))
    }
}



/// Attempts to convert a `JsValue` into a `Hashmap<String, String>`.
pub fn js_as_string_hashmap(js_obj: JsValue, func_name: &str, param_name: &str) -> Result<HashMap<String, String>, JsValue> {
    //  We'll use an object-as-dictionary API to avoid using `Reflect::*` methods.
    //  They're NOT present in many mobile browsers at the time of writing.
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen]
        type ObjectAsDictionary;
    
        #[wasm_bindgen(js_namespace = Object, js_name = assign)]
        fn dict_from_js(js_val: JsValue) -> ObjectAsDictionary;
    
        #[wasm_bindgen(js_namespace = Object, js_name = keys)]
        fn dict_keys(o_dict: &ObjectAsDictionary) -> Array;
    
        #[wasm_bindgen(method, structural, indexing_getter)]
        fn get_value_for(this: &ObjectAsDictionary, key: &JsValue) -> Option<String>;
    }
    //
    let dict        = dict_from_js(js_obj);
    let keys        = dict_keys(&dict);
    let mut hash    = HashMap::<String, String>::with_capacity(keys.length() as usize);
    for key in keys.iter() {
        let val     = dict.get_value_for(&key);
        let key_str = JsString::from(key)
            .as_string()
            .ok_or_else(|| JsValue::from_str(&format!(
                "{}: Error: param '{}' is not an object.",
                func_name,
                param_name
            )))?;
        let val = JsString::from(
            val
                .ok_or_else(|| JsValue::from_str(&format!(
                    "{}: Error: cannot extract property '{}' from param '{}.",
                    func_name,
                    key_str,
                    param_name
                )))?
        )
            .as_string()
            .ok_or_else(|| JsValue::from_str(&format!(
                "{}: Error: failed to process value of property '{}' from param '{}.",
                func_name,
                key_str,
                param_name
            )))?;
        hash.insert(key_str, val);
    }
    Ok(hash)
}


// /// Attempts to convert a `JsValue` into a `Hashmap<String, String>`.
// #[allow(dead_code)]
// pub fn js_as_string_hashmap_old(js_val: JsValue, func_name: &str, param_name: &str) -> Result<HashMap<String, String>, JsValue> {
//     let obj = js_val.dyn_into::<Object>()
//         .map_err(|_| JsValue::from_str(&format!(
//             "{}: Error: param '{}' is not an object.",
//             func_name,
//             param_name
//         )))?;
//     let keys        = Object::keys(&obj);
//     let mut hash    = HashMap::<String, String>::with_capacity(keys.length() as usize);
//     for key in keys.iter() {
//         let val     = Reflect::get(&obj, &key);
//         let key_str = JsString::from(key)
//             .as_string()
//             .ok_or_else(|| JsValue::from_str(&format!(
//                 "{}: Error: param '{}' is not an object.",
//                 func_name,
//                 param_name
//             )))?;
//         let val = JsString::from(
//             val
//                 .map_err(|_| JsValue::from_str(&format!(
//                     "{}: Error: cannot extract property '{}' from param '{}.",
//                     func_name,
//                     key_str,
//                     param_name
//                 )))?
//         )
//             .as_string()
//             .ok_or_else(|| JsValue::from_str(&format!(
//                 "{}: Error: failed to process value of property '{}' from param '{}.",
//                 func_name,
//                 key_str,
//                 param_name
//             )))?;
//         hash.insert(key_str, val);
//     }
//     Ok(hash)
// }
