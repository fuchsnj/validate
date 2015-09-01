extern crate regex;

use std::fmt::Display;
use regex::Regex;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo, Add};

#[cfg(test)]
mod test;

pub type ValidationResult = Result<(), String>;


pub struct Schema<'a, T>{
	validator: Box<Rule<T> + 'a>
}

impl<'a, T: 'a> Schema<'a, T>{
	pub fn new() -> Schema<'a, T>{
		Schema{
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
			validator: Box::new(move |data: &T|{
				try!(self.validate(data));
				rule.validate(data)
			})
		}
	}
}

impl<'a> Schema<'a, &'a str>{
	pub fn match_regex(self, pattern: &str) -> Self{	
		//TODO: remove unwrap and use compile time regex check
		let regex = Regex::new(pattern).unwrap();
		self.rule(move |input: &&str|{
			match regex.is_match(input){
				true => Ok(()),
				false => Err("failed to match regex".to_string())
			}
		})
	}
	
	pub fn email(self) -> Self{	
		self.match_regex(r"^[^@]+@[^@]+\.[^@]+$")
	}
	
	pub fn length<A>(self, bounds: A) -> Self
	where A: Bounds<usize>{
		match bounds.get_bounds(){
			Bound::Unbounded => self,
			Bound::RangeFrom(a) => self.rule(move |input: &&str|{
				match input.len() >= a {
					true => Ok(()),
					false => Err(format!("must be at least {} characters long", a))
				}
			}),
			Bound::RangeTo(b)=> self.rule(move |input: &&str|{
				match input.len() < b {
					true => Ok(()),
					false => Err(format!("must be less than {} characters long", b))
				}
			}),
			Bound::Range(a,b) => self.rule(move |input: &&str|{
				match input.len() >= a && input.len() < b {
					true => Ok(()),
					false => Err(format!("must be {} to {} characters long", a, b-1))
				}
			}),
			Bound::Exact(a) => self.rule(move |input: &&str|{
				match input.len() == a {
					true => Ok(()),
					false => Err(format!("must be {} characters long", a))
				}
			})
		}
	} 
}

impl<'a, T> Schema<'a, T>
where T: PartialOrd + 'a + Display{
	pub fn range<A>(self, bounds: A) -> Self
	where A: Bounds<T>{
		match bounds.get_bounds(){
			Bound::Unbounded => self,
			Bound::RangeFrom(a) => self.rule(move |input: &T|{
				match input >= &a {
					true => Ok(()),
					false => Err(format!("must be >= {}", a))
				}
			}),
			Bound::RangeTo(b)=> self.rule(move |input: &T|{
				match input < &b {
					true => Ok(()),
					false => Err(format!("must be < {}", b))
				}
			}),
			Bound::Range(a,b) => self.rule(move |input: &T|{
				match input >= &a && input < &b {
					true => Ok(()),
					false => Err(format!("must >= {} and < {}", a, b))
				}
			}),
			Bound::Exact(a) => self.rule(move |input: &T|{
				match input == &a {
					true => Ok(()),
					false => Err(format!("must be {}", a))
				}
			})
		}
	}
}

pub trait Rule<T>{
	fn validate(&self, input:&T) -> Result<(), String>;
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