extern crate regex;

mod rule;
mod bound;

pub mod rules;

pub use rule::{Rule, ValidationResult, Error};

pub trait Validate: 'static {
	fn get_validation_rule(&self) -> Rule<Self>;

	fn validate(&self) -> ValidationResult {
		self.get_validation_rule().validate(self)
	}
}
