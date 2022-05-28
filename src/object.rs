#[derive(Debug)]
pub enum MalObject {
    Nil,
    Bool(bool),
    Int(i64),
    Str(String),
    Symbol(String),
    List(Vec<MalObject>),
}

impl MalObject {
    pub fn new() -> Self {
	MalObject::Nil
    }

    pub fn new_boolean(bool_val: bool) -> Self {
	MalObject::Bool(bool_val)
    }

    pub fn new_symbol(sym: String) -> Self {
	MalObject::Symbol(sym)
    }

    pub fn new_list(objects: Vec<MalObject>) -> Self {
	MalObject::List(objects)
    }
}
