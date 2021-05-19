use std::fmt::{self, Write};

use crate::core::{Obj, Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

fn write_pair<'a>(
    f: &mut fmt::Formatter<'_>,
    num_indents: usize,
    key: &str,
    value: &Value<'a>,
) -> fmt::Result {
    // Write the indented key
    write!(f, "{}\"{}\"", multiple_char('\t', num_indents), key)?;

    // Followed by the value
    if value.is_str() {
        f.write_char('\t')?;
    } else {
        f.write_char('\n')?;
    }
    value.write_indented(f, num_indents)?;

    f.write_char('\n')
}

fn write_obj<'a>(f: &mut fmt::Formatter<'_>, num_indents: usize, obj: &Obj<'a>) -> fmt::Result {
    for (key, values) in obj.iter() {
        for value in values {
            write_pair(f, num_indents, key, value)?;
        }
    }

    Ok(())
}

impl<'a> fmt::Display for Vdf<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_indented(f, 0)
    }
}

// TODO: this needs to handle escaping special characters like \", \\, \n, etc.
// TODO: make a note that keys with no corresponding values get skipped
impl<'a> Vdf<'a> {
    fn write_indented(&self, f: &mut fmt::Formatter<'_>, num_indents: usize) -> fmt::Result {
        write_pair(f, num_indents, &self.key, &self.value)
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
                writeln!(f, "{}{{", multiple_char('\t', num_indents))?;
                write_obj(f, num_indents + 1, obj)?;
                // obj.write_indented(f, num_indents + 1)?;
                write!(f, "{}}}", multiple_char('\t', num_indents))
            }
        }
    }
}
