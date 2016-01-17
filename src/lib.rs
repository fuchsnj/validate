extern crate regex;

use std::fmt::Display;
use regex::Regex;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

#[cfg(test)]
mod test;

pub type ValidationResult = Result<(), Error>;

#[derive(Debug)]
pub struct Error{
	message: String
}
impl Error{
	pub fn new(msg: &str) -> Error{
		Error{
			message: msg.to_owned()
		}
	}
	pub fn get_message(&self) -> &str{
		&self.message
	}
}

pub struct Schema<'a, T>{
	validator: Box<Rule<T> + 'a>,
	name: String
}

impl<'a, T: 'a> Schema<'a, T>{
	pub fn new(name: &str) -> Schema<'a, T>{
		Schema{
			name: name.to_owned(),
			validator: Box::new(|_: &T|{
				Ok(())
			})
		}
	}
	
	pub fn validate(&self, data: &T) -> ValidationResult{
		self.validator.validate(data)
	}
	
	pub fn rule<R>(self, rule: R) -> Schema<'a, T> 
	where R: Rule<T> + 'a{
		Schema{
			name: self.name.clone(),
			validator: Box::new(move |data: &T|{
				try!(self.validate(data));
				rule.validate(data)
			})
		}
	}
}

impl<'a> Schema<'a, &'a str>{
	pub fn whitelist_chars(self, whitelist: &str) -> Self{
		let whitelist = whitelist.to_owned();
		let name = self.name.clone();
		self.rule(move |input: &&str|{
			for c in input.chars(){
				if !whitelist.contains(c){
					return Err(Error::new(&format!("{} cannot contain the character '{}'", name, c)))
				}
			}
			Ok(())
		})
	}
	pub fn match_regex(self, pattern: &str, description: &str) -> Self{	
		//TODO: remove unwrap and use compile time regex check
		let regex = Regex::new(pattern).unwrap();
		let name = self.name.clone();
		let description = description.to_owned();
		self.rule(move |input: &&str|{
			match regex.is_match(input){
				true => Ok(()),
				false => Err(Error::new(&format!("{} must be {}", name, description)))
			}
		})
	}
	
	pub fn email(self) -> Self{	
		self.match_regex(r"^[^@]+@[^@]+\.[^@]+$", "a valid email")
	}
	
	pub fn length<A>(self, bounds: A) -> Self
	where A: Bounds<usize>{
		let name = self.name.clone();
		match bounds.get_bounds(){
			Bound::Unbounded => self,
			Bound::RangeFrom(a) => self.rule(move |input: &&str|{
				match input.len() >= a {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be at least {} characters long", name, a)))
				}
			}),
			Bound::RangeTo(b)=> self.rule(move |input: &&str|{
				match input.len() < b {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be less than {} characters long", name, b)))
				}
			}),
			Bound::Range(a,b) => self.rule(move |input: &&str|{
				match input.len() >= a && input.len() < b {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be {} to {} characters long", name, a, b-1)))
				}
			}),
			Bound::Exact(a) => self.rule(move |input: &&str|{
				match input.len() == a {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be {} characters long", name, a)))
				}
			})
		}
	} 
}

impl<'a, T> Schema<'a, T>
where T: PartialOrd + 'a + Display{
	pub fn range<A>(self, bounds: A) -> Self
	where A: Bounds<T>{
		let name = self.name.clone();
		match bounds.get_bounds(){
			Bound::Unbounded => self,
			Bound::RangeFrom(a) => self.rule(move |input: &T|{
				match input >= &a {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be >= {}", name, a)))
				}
			}),
			Bound::RangeTo(b)=> self.rule(move |input: &T|{
				match input < &b {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must be < {}", name, b)))
				}
			}),
			Bound::Range(a,b) => self.rule(move |input: &T|{
				match input >= &a && input < &b {
					true => Ok(()),
					false => Err(Error::new(&format!("{} must >= {} and < {}", name, a, b)))
				}
			}),
			Bound::Exact(a) => self.rule(move |input: &T|{
				match input == &a {
					true => Ok(()),
					false => Err(Error::new(&format!("{}, must be {}", name, a)))
				}
			})
		}
	}
}

pub trait Rule<T>{
	fn validate(&self, input:&T) -> ValidationResult;
}

impl<F, Type> Rule<Type> for F
where F: Fn(&Type) -> ValidationResult{
	fn validate(&self, input:&Type) -> ValidationResult{
		(*self)(input)
	}
}

pub trait Bounds<T>{
	fn get_bounds(&self) -> Bound<T>;
}

pub enum Bound<T>{
	Exact(T),
	Range(T, T),
	RangeFrom(T),
	RangeTo(T),
	Unbounded
}

impl Bounds<usize> for usize{
	fn get_bounds(&self) -> Bound<usize>{
		Bound::Exact(*self)
	}
}

impl Bounds<usize> for Range<usize>{
	fn get_bounds(&self) -> Bound<usize>{
		Bound::Range(self.start, self.end)
	}
}

impl Bounds<usize> for RangeFrom<usize>{
	fn get_bounds(&self) -> Bound<usize>{
		Bound::RangeFrom(self.start)
	}
}

impl Bounds<usize> for RangeTo<usize>{
	fn get_bounds(&self) -> Bound<usize>{
		Bound::RangeTo(self.end)
	}
}

impl Bounds<usize> for RangeFull{
	fn get_bounds(&self) -> Bound<usize>{
		Bound::Unbounded
	}
}