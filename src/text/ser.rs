use std::fmt::{self, Write};

use crate::core::{Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

impl<'a> fmt::Display for Vdf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl<'a> Vdf<'a> {
    fn write_indented(&self, f: &mut fmt::Formatter<'_>, num_indents: usize) -> fmt::Result {
        for (key, values) in self.iter() {
            for value in values {
                // Write the indented key
                write!(f, "{}\"{}\"", multiple_char('\t', num_indents), key)?;

                // Followed by the value
                if value.is_str() {
                    f.write_char('\t')?;
                } else {
                    f.write_char('\n')?;
                }
                value.write_indented(f, num_indents)?;

                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl<'a> fmt::Display for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

impl<'a> Value<'a> {
    fn write_indented(&self, f: &mut fmt::Formatter<'_>, num_indents: usize) -> fmt::Result {
        // Only `Obj` gets indented
        match self {
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Obj(obj) => {
                write!(f, "{}{{\n", multiple_char('\t', num_indents))?;
                obj.write_indented(f, num_indents + 1)?;
                write!(f, "{}}}", multiple_char('\t', num_indents))
            }
        }
    }
}
