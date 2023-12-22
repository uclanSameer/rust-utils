use std::{collections::HashMap, fs::File, io::Write};

use serde_json::Value;

pub fn json_to_struct(in_file_name: &str, struct_name: &str) {
    let json = std::fs::read_to_string(in_file_name).unwrap();
    let value: Value = serde_json::from_str(&json).unwrap();

    let struct_file_name = struct_name.to_string() + ".rs";
    let _file = std::fs::File::create(&struct_file_name).unwrap();

    // open file in append mode
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&struct_file_name)
        .unwrap();

    // convert first character to uppercase
    let struct_name = struct_name
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();

    convert_to_struct(&value, struct_name.as_str(), &mut file);
}

pub fn convert_to_struct(
    value: &Value,
    name_of_struct: &str,
    file_out: &mut File,
) -> HashMap<String, String> {
    let mut struct_map: HashMap<String, String> = std::collections::HashMap::new();
    value.as_object().unwrap().iter().for_each(|(k, v)| {
        if v.is_string() {
            struct_map.insert(k.to_string(), "String".to_string());
        } else if v.is_number() {
            match v.as_i64() {
                Some(_) => {
                    struct_map.insert(k.to_string(), "i64".to_string());
                }
                None => {
                    struct_map.insert(k.to_string(), "f64".to_string());
                }
            }
        } else if v.is_boolean() {
            struct_map.insert(k.to_string(), "bool".to_string());
        } else if v.is_null() {
        } else if v.is_object() {
            for_object(k, v, file_out, &mut struct_map);
        } else if v.is_array() {
            for_array(v, &mut struct_map, k, file_out);
        }
    });

    let struct_str = serde_json::to_string(&struct_map).unwrap();
    // remove " from string
    let mut struct_str = struct_str.replace("\"", "");

    struct_str = "struct ".to_string() + name_of_struct + &struct_str;

    struct_str = indent_string(struct_str, 4);
    // write to file
    file_out.write(struct_str.as_bytes()).unwrap();
    struct_map
}

/**
 * This function is used to convert object to struct
 *
 * Eg:
 *  {
 *     "planet": {"name": "Earth", "radius": 6371}
 * }
 *
 * output:
 *  struct Planet {
 *    name: String,
 *   radius: i64
 * }
 */
fn for_object(k: &String, v: &Value, file: &mut File, struct_map: &mut HashMap<String, String>) {
    // make first letter uppercase of key
    let inner_name_of_struct = k
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();
    convert_to_struct(v, &inner_name_of_struct, file);

    struct_map.insert(k.to_string(), inner_name_of_struct);
}

/**
 * This function is used to convert array to struct
 *
 * eg: {"players": [1, 2, 3]}
 *
 * output:
 *  struct NameOfStruct {
 *     players: Vec<i64>
 *}
 */
fn for_array(v: &Value, struct_map: &mut HashMap<String, String>, k: &String, file_out: &mut File) {
    let get_first_element = v.as_array().unwrap().get(0).unwrap();

    if get_first_element.is_string() {
        struct_map.insert(k.to_string(), "Vec<String>".to_string());
    } else if get_first_element.is_number() {
        match get_first_element.as_i64() {
            Some(_) => {
                struct_map.insert(k.to_string(), "Vec<i64>".to_string());
            }
            None => {
                struct_map.insert(k.to_string(), "Vec<f64>".to_string());
            }
        }
    } else if get_first_element.is_boolean() {
        struct_map.insert(k.to_string(), "Vec<bool>".to_string());
    } else if get_first_element.is_null() {
    } else if get_first_element.is_object() {
        // make first letter uppercase of key
        let inner_name_of_struct = k
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        convert_to_struct(get_first_element, &inner_name_of_struct, file_out);

        struct_map.insert(k.to_string(), format!("Vec<{}>", inner_name_of_struct));
    }
}

// TODO: implementation missing
fn indent_string(input: String, indent: usize) -> String {
    let mut output = String::new();
    for _ in 0..indent {
        output.push_str(" ");
    }
    output.push_str(&input);
    output
}
