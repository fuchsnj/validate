use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct ValidationError(pub String);

pub type ValidationResult = Result<(), ValidationError>;

pub struct Rule<T: ? Sized> {
	rule: Box<Fn(&T) -> ValidationResult>
}

impl<T: ? Sized + 'static> Rule<T> {
	pub fn validate(&self, input: &T) -> ValidationResult {
		(*self.rule)(input)
	}

	pub fn nest<'b, R, F, U>(name: &str, get: F, rule: R) -> Self
		where
				R: Into<Rule<U>>,
				F: Fn(&T) -> &U + 'static,
				U: ? Sized + 'static {
		let name = name.to_owned();
		let converted_rule = rule.into();
		Rule::from(move |input: &T| {
			let mapped_input = get(input);
			converted_rule.validate(&mapped_input).map_err(|err: ValidationError| {
				ValidationError(name.clone() + " " + &err.0)
			})
		})
	}

	pub fn map<'b, R, F, U>(name: &str, get: F, rule: R) -> Self
		where
				R: Into<Rule<U>>,
				F: Fn(&T) -> U + 'static,
				U: 'static {
		let name = name.to_owned();
		let converted_rule = rule.into();
		Rule::from(move |input: &T| {
			let mapped_input = get(input);
			converted_rule.validate(&mapped_input).map_err(|err: ValidationError| {
				ValidationError(name.clone() + " " + &err.0)
			})
		})
	}
}

impl<T: ? Sized, F: Fn(&T) -> ValidationResult + 'static> From<F> for Rule<T> {
	fn from(func: F) -> Self {
		Rule {
			rule: Box::new(func)
		}
	}
}

impl<T: ? Sized + 'static, R: Into<Rule<T>>> Add<R> for Rule<T> {
	type Output = Rule<T>;

	fn add(self, rule: R) -> Self::Output {
		let rule = rule.into();
		Rule::from(move |input: &T| {
			self.validate(input)?;
			rule.validate(input)
		})
	}
}