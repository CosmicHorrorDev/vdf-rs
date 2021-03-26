#[derive(Debug)]
pub struct Vdf<'a>(pub VdfPair<'a>);

#[derive(Debug)]
pub struct VdfPair<'a>(pub &'a str, pub VdfValue<'a>);

#[derive(Debug)]
pub enum VdfValue<'a> {
    Str(&'a str),
    Obj(Vec<VdfPair<'a>>),
}
