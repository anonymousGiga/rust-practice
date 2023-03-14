use std::collections::HashMap;
pub type Data = HashMap<String, u32>;
pub trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}
