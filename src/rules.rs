use Schema;
use std::fmt::Display;
use regex::Regex;

pub fn min_length(len: usize) -> Box<Schema<String>>{
	Box::new(move |input:&String|{
		match input.len() >=len{
			true => Ok(()),
			false => Err("fail".to_string())
		}
	})
}

pub fn min<T>(min: T) -> Box<Schema<T>>
where T: PartialOrd + 'static + Display{
	Box::new(move |input:&T|{
		match input >= &min {
			true => Ok(()),
			false => Err(format!("must be >= {}", min).to_string())
		}
	})
}

pub fn max<T>(max: T) -> Box<Schema<T>>
where T: PartialOrd + 'static + Display{
	Box::new(move |input:&T|{
		match input <= &max {
			true => Ok(()),
			false => Err(format!("must be <= {}", max).to_string())
		}
	})
}

pub fn match_regex(pattern: &str) -> Box<Schema<String>>{	
	//TODO: remove unwrap and use compile time regex check
	let regex = Regex::new(pattern).unwrap();

	Box::new(move |input:&String|{
		match regex.is_match(input){
			true => Ok(()),
			false => Err("failed to match regex".to_string())
		}
	})
}

pub fn range<T>(min_value:T, max_value: T) -> Box<Schema<T>>
where T: PartialOrd + 'static + Display{
	min(min_value) + max(max_value)
}

pub fn email() -> Box<Schema<String>>{	
	match_regex(r"^[^@]+@[^@]+\.[^@]+$")
}
