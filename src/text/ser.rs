use std::fmt;

use crate::common::{Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

impl fmt::Display for Vdf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Vdf {
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Value {
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
