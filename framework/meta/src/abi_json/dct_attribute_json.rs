use dharitri_sc::abi::{DctAttributeAbi, TypeName};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DctAttributeJson {
    pub ticker: String,
    #[serde(rename = "type")]
    pub ty: TypeName,
}

impl From<&DctAttributeAbi> for DctAttributeJson {
    fn from(attr: &DctAttributeAbi) -> Self {
        DctAttributeJson {
            ticker: attr.ticker.to_owned(),
            ty: attr.ty.clone(),
        }
    }
}
