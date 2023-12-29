struct VariableParam {
    name: String,
    value: String,
}

struct PositionalParam {
    position: String,
    value: String,
}

struct SpecialParam {
    identifier: String,
    value: String,
}

enum Parameter {
    Variable(VariableParam),
    Positional(PositionalParam),
    Special(SpecialParam),
}
