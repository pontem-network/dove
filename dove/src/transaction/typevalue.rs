use anyhow::Error;
use dsl::parser::types::{Value, Value_};

/// Convert Vec<Value> => Vec<String>
pub trait VecValueToVecString {
    /// Convert Vec<Value> => Vec<String>
    fn to_string(&self) -> Result<Vec<String>, Error>;
}
impl VecValueToVecString for Vec<Value> {
    fn to_string(&self) -> Result<Vec<String>, Error> {
        self.iter()
            .map(|value| {
                let str = match &value.value {
                    Value_::Var(v) => v.clone(),
                    Value_::Address(v) => v.to_string(),
                    Value_::Bool(v) => if *v { "true" } else { "false" }.to_string(),
                    Value_::Num(v) => v.to_string(),
                    Value_::Vec(v) => format!("[{}]", v.to_string()?.join(",")),
                    _ => anyhow::bail!("The type is not supported"),
                };
                Ok(str)
            })
            .collect::<Result<Vec<String>, Error>>()
    }
}
