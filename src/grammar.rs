pub struct VariableParam {
    name: String,
    value: String,
}

pub struct PositionalParam {
    position: String,
    value: String,
}

pub struct SpecialParam {
    identifier: String,
    value: String,
}

pub enum Parameter {
    Variable(VariableParam),
    Positional(PositionalParam),
    Special(SpecialParam),
}

pub enum Name {
    Command(String),
    Variable(String),
    Builtin(String),
}

pub struct ControlOperator {}

impl ControlOperator {
    pub const OR: &'static str = "||";
    pub const AND: &'static str = "&&";
    pub const SEMICOLON: &'static str = ";";
    pub const NEW_LINE: &'static str = "\n";
    pub const PIPE: &'static str = "|";
}

pub enum Word {
    Name,
    ControlOperator,
}
