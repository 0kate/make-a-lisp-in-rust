use crate::object::MalObject;
use crate::tokenizer::Tokenizer;

pub struct Parser<'a> {
    tokenizer: &'a mut Tokenizer,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: &'a mut Tokenizer) -> Self {
	Parser {
	    tokenizer,
	}
    }

    fn parse_form(&mut self) -> MalObject {
	match self.tokenizer.peek() {
	    Some(token) => {
		match &token as &str {
		    "(" => self.parse_list(),
		    ")" => {
			self.tokenizer.next();
			self.parse_form()
		    },
		    _ => self.parse_atom(),
		}
	    },
	    None => {
		MalObject::new()
	    },
	}
    }

    fn parse_list(&mut self) -> MalObject {
	let mut objects: Vec<MalObject> = Vec::new();

	loop {
	    let token = self.tokenizer.next();
	    if token == None {
		break;
	    }
	    objects.push(self.parse_form());
	}
	MalObject::new_list(objects)
    }

    fn parse_atom(&self) -> MalObject {
	let token = self.tokenizer.peek();
	match token {
	    Some(token) => {
	        match &token as &str {
		    "nil" => MalObject::new(),
		    "false" => MalObject::new_boolean(false),
		    "true" => MalObject::new_boolean(true),
		    _ => {
			MalObject::new_symbol(token)
		    },
		}
	    },
	    None => {
		MalObject::new()
	    }
	}
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = MalObject;

    fn next(&mut self) -> Option<Self::Item> {
	if self.tokenizer.has_next() {
	    Some(self.parse_form())
	} else {
	    None
	}
    }
}
