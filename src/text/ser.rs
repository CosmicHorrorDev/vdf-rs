use std::fmt;

use crate::common::{Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

impl<'a> fmt::Display for Vdf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl<'a> Vdf<'a> {
    pub fn write_indented(&self, f: &mut fmt::Formatter<'_>, num_indents: usize) -> fmt::Result {
        for (key, values) in self.iter() {
            for value in values {
                write!(f, "{}\"{}\"", multiple_char('\t', num_indents), key)?;
                match value {
                    Value::Str(s) => write!(f, "\t\"{}\"", s)?,
                    Value::Obj(obj) => {
                        write!(f, "\n{}{{\n", multiple_char('\t', num_indents))?;
                        obj.write_indented(f, num_indents + 1)?;
                        write!(f, "{}}}", multiple_char('\t', num_indents))?;
                    }
                }
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Obj(obj) => {
                f.write_str("{\n")?;
                obj.write_indented(f, 1)?;
                f.write_str("}")
            }
        }
    }
}
