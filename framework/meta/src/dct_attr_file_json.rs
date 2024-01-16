use serde::Serialize;
use std::{fs::File, io::Write, path::Path};

use crate::abi_json::DctAttributeAbiJson;

pub fn create_new_dct_attr_file(
    json: &DctAttributeAbiJson,
    path: impl AsRef<Path>,
    file_name_arg: &str,
) {
    let abi_string = serialize_dct_attribute_json(json);
    let mut file_name = file_name_arg.replace(|c: char| !c.is_ascii_alphanumeric(), "_");
    file_name.push_str(".dct-abi.json");
    let abi_file_path = path.as_ref().join(file_name);
    let mut abi_file = File::create(abi_file_path).unwrap();
    write!(abi_file, "{abi_string}").unwrap();
}

pub fn serialize_dct_attribute_json(json: &DctAttributeAbiJson) -> String {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    json.serialize(&mut ser).unwrap();
    let mut serialized = String::from_utf8(ser.into_inner()).unwrap();
    serialized.push('\n');
    serialized
}
