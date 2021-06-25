use std::{
    fmt::{self, Write},
};

use crate::{Obj, Value, Vdf};

fn multiple_char(c: char, amount: usize) -> String {
    std::iter::repeat(c).take(amount).collect()
}

fn write_escaped_str(f: &mut fmt::Formatter<'_>, s: &str) -> fmt::Result {
    f.write_char('"')?;

    for c in s.chars() {
        match c {
            '\n' => f.write_str(r"\n"),
            '\r' => f.write_str(r"\r"),
            '\t' => f.write_str(r"\t"),
            '\"' => f.write_str(r#"\""#),
            '\\' => f.write_str(r"\\"),
            reg => f.write_char(reg),
        }?
    }

    f.write_char('"')
}

fn write_pair<'a>(
    f: &mut fmt::Formatter<'_>,
    num_indents: usize,
    key: &str,
    value: &Value<'a>,
) -> fmt::Result {
    // Write the indented key
    f.write_str(&multiple_char('\t', num_indents))?;
    write_escaped_str(f, key)?;

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
            Value::Str(s) => write_escaped_str(f, s),
            Value::Obj(obj) => {
                writeln!(f, "{}{{", multiple_char('\t', num_indents))?;
                write_obj(f, num_indents + 1, obj)?;
                // obj.write_indented(f, num_indents + 1)?;
                write!(f, "{}}}", multiple_char('\t', num_indents))
            }
        }
    }
}
