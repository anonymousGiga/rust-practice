use crate::formatter::Formatter;
use std::collections::HashMap;

pub struct Report;

impl Report {
    pub fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        g.format(&data, s);
    }
}
