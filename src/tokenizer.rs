use regex::Regex;

pub struct Tokenizer {
    index: usize,
    tokens: Vec<String>,
}

impl Tokenizer {
    pub fn new(exp_str: String) -> Self {
	Tokenizer {
	    index: 0,
	    tokens: Tokenizer::do_tokenize(exp_str),
	}
    }

    fn do_tokenize(exp_str: String) -> Vec<String> {
	let reg = Regex::new(r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###).unwrap();
	let mut tokens = Vec::new();

	// TODO: be declarative
	for cap in reg.captures_iter(&exp_str.to_owned()) {
	    tokens.push(String::from(&cap[1]));
	}
	tokens
    }

    pub fn peek(&self) -> Option<String> {
	if self.has_next() {
	    Some(self.tokens[self.index].clone())
	} else {
	    None
	}
    }

    pub fn next(&mut self) -> Option<String> {
	self.index += 1;
	self.peek()
    }

    pub fn has_next(&self) -> bool {
	self.index < self.tokens.len()
    }
}
