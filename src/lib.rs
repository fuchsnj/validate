extern crate regex;

use std::str::FromStr;
use std::ops::Add;

#[cfg(test)]
mod test;

pub mod rules;


pub type ValidationResult = Result<(), String>;


pub trait ConvertTo<Type>{
	fn convert(&self) -> Result<Type, String>;
}

pub trait Schema<Type>{
	fn validate(&self, input:&Type) -> Result<(), String>;

	fn convert(&self, input: &ConvertTo<Type>) -> Result<Type, String>{
		let convert_result:Result<Type, String> = input.convert();
		let output = try!(convert_result);
		try!(self.validate(&output));
		Ok(output)
	}
}

impl<T> Add for Box<Schema<T>>{
	type Output = Box<Schema<T>>;

	fn add(self, other: Box<Schema<T>>) -> Box<Schema<T>>{
		Box::new(move |input:&T|{
			try!(self.validate(input));
			other.validate(input)
		})
	}
}


impl<F, Type> Schema<Type> for F
where F: Fn(&Type) -> ValidationResult{
	fn validate(&self, input:&Type) -> ValidationResult{
		(*self)(input)
	}
}

impl<'a, T> ConvertTo<T> for &'a str
where T: FromStr{
	fn convert(&self) -> Result<T, String>{
		T::from_str(self).map_err(|_|{
			"parse error".to_string()
		})
	}
}

// impl<A, B> ConvertTo<B> for A
// where B: From<A>, A: Clone{
// 	fn convert(&self) -> Result<B, String>{
// 		Ok(self.clone().into())
// 	}
// }