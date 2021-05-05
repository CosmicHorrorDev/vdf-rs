use std::fmt;

use crate::common::{Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

// TODO: this can be implemented for just `Vdf` instead of needing to be owned
impl<'a> fmt::Display for Vdf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'a> Vdf<'a> {
    pub fn to_string(&self) -> String {
        self.to_indented_string(0)
    }

    pub fn to_indented_string(&self, num_indents: usize) -> String {
        let mut fmt = String::new();

        for (key, values) in self.iter() {
            let key_fmt = format!("{}\"{}\"", multiple_char('\t', num_indents), key);
            for value in values.iter() {
                fmt += &format!("{}{}", key_fmt, value.to_indented_string(num_indents));
            }
        }
        fmt
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl<'a> Value<'a> {
    pub fn to_string(&self) -> String {
        self.to_indented_string(0)
    }

    pub fn to_indented_string(&self, num_indents: usize) -> String {
        match self {
            Value::Str(s) => format!("\t\"{}\"\n", s),
            Value::Obj(obj) => {
                let indent = multiple_char('\t', num_indents);
                format!(
                    "\n{}{{\n{}{}}}\n",
                    indent,
                    obj.to_indented_string(num_indents + 1),
                    indent
                )
            }
        }
    }
}
