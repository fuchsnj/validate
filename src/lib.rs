extern crate regex;

mod rule;
mod bound;

pub mod rules;

pub use rule::{Rule, ValidationResult, Error};

pub trait Validate: 'static {
	fn validate(&self) -> ValidationResult;
}

impl Validate for () {
	fn validate(&self) -> ValidationResult {
		Ok(())
	}
}

impl<T: Validate> Validate for Option<T> {
	fn validate(&self) -> ValidationResult {
		match *self {
			Some(ref data) => data.validate(),
			None => Ok(())
		}
	}
}

impl<T: Validate> Validate for Box<T> {
	fn validate(&self) -> ValidationResult {
		self.as_ref().validate()
	}
}
