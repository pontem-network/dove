#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum ResourceChangeOp {
    SetValue { values: Vec<u8> },
    Delete,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceType {
    pub address: String,
    pub module: String,
    pub name: String,
    pub ty_args: Vec<String>,
    pub layout: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ResourceChange {
    pub ty: ResourceType,
    pub op: ResourceChangeOp,
}

impl ResourceChange {
    pub fn new(ty: impl Into<ResourceType>, op: impl Into<ResourceChangeOp>) -> ResourceChange {
        ResourceChange {
            ty: ty.into(),
            op: op.into(),
        }
    }
}
