// ༼ つ ◕_◕ ༽つ  #![forbid(unsafe_code)]

use std::collections::HashMap;
use std::mem::transmute;

#[derive(Debug, Default)]
pub struct InternTable {
    table: HashMap<String, &'static str>,
}

impl InternTable {
    pub fn push(&mut self, val: String) -> &'static str {
        self.table
            .entry(val)
            .or_insert_with_key(|key| unsafe { transmute(key.as_str()) })
    }
}

impl Drop for InternTable {
    fn drop(&mut self) {
        for (_, rf) in self.table.drain() {
            let _: &str = unsafe { transmute(rf) };
        }
    }
}
