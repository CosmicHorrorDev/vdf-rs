use std::fmt;

use crate::common::{Pair, Value, Vdf};

trait StringExt {
    fn push_multiple(&mut self, c: char, amount: u16);
}

impl StringExt for String {
    fn push_multiple(&mut self, c: char, amount: u16) {
        for _ in 0..amount {
            self.push(c);
        }
    }
}

impl<'a> fmt::Display for Vdf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'a> Vdf<'a> {
    pub fn to_string(&self) -> String {
        // self.0.to_string(0)
        self.0
            .iter()
            .map(|pair| pair.to_string(0))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl<'a> Pair<'a> {
    pub fn to_string(&self, num_tabs: u16) -> String {
        let mut formatted = String::new();

        formatted.push_multiple('\t', num_tabs);
        formatted.push_str(&format!("\"{}\"", self.key()));
        formatted.push_str(&self.value().to_string(num_tabs));

        formatted
    }
}

impl<'a> Value<'a> {
    pub fn to_string(&self, num_tabs: u16) -> String {
        let mut formatted = String::new();

        match self {
            Value::Str(s) => {
                formatted.push_str(&format!("\t\"{}\"\n", s));
            }
            Value::Obj(obj) => {
                formatted.push('\n');
                formatted.push_multiple('\t', num_tabs);
                formatted.push_str("{\n");

                for pair in obj {
                    formatted.push_str(&pair.to_string(num_tabs + 1));
                }

                formatted.push_multiple('\t', num_tabs);
                formatted.push_str("}\n");
            }
        }

        formatted
    }
}
