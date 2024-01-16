use std::collections::BTreeMap;

use dharitri_sc::abi::DctAttributeAbi;
use serde::{Deserialize, Serialize};

use super::{convert_type_descriptions_to_json, DctAttributeJson, TypeDescriptionJson};

/// Represents an entire DCT attribute ABI file. The type descriptions only show up here.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DctAttributeAbiJson {
    pub dct_attribute: DctAttributeJson,

    #[serde(default)]
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub types: BTreeMap<String, TypeDescriptionJson>,
}

impl DctAttributeAbiJson {
    pub fn new(attr: &DctAttributeAbi) -> Self {
        DctAttributeAbiJson {
            dct_attribute: DctAttributeJson::from(attr),
            types: convert_type_descriptions_to_json(&attr.type_descriptions),
        }
    }
}
