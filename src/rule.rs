use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Error {
	message: String
}

impl Error {
	pub fn get_message(&self) -> String {
		self.message.clone()
	}
}

impl<S: Into<String>> From<S> for Error {
	fn from(msg: S) -> Self {
		Error { message: msg.into() }
	}
}


pub type ValidationResult = Result<(), Error>;

pub struct Rule<T: ? Sized> {
	rule: Box<Fn(&T) -> ValidationResult>
}

impl<T: 'static> Rule<T> {
	pub fn map<'b, F, U>(self, get: F) -> Rule<U>
		where F: Fn(&U) -> T + 'static,
		      U: ? Sized + 'static {
		Rule::from(move |input: &U| {
			let mapped_input = get(input);
			self.validate(&mapped_input)
		})
	}
}

impl<T: ? Sized + 'static> Rule<T> {
	pub fn validate(&self, input: &T) -> ValidationResult {
		(*self.rule)(input)
	}

	pub fn nest<'b, R, F, U>(get: F, rule: R) -> Self
		where
				R: Into<Rule<U>>,
				F: Fn(&T) -> &U + 'static,
				U: ? Sized + 'static {
		let converted_rule = rule.into();
		Rule::from(move |input: &T| {
			let mapped_input = get(input);
			converted_rule.validate(&mapped_input)
		})
	}


	pub fn name(self, name: &str) -> Self {
		let name = name.to_owned();
		Rule::from(move |input: &T| {
			self.validate(&input).map_err(|err: Error| {
				(name.clone() + " " + &err.get_message()).into()
			})
		})
	}
}

impl<T, F> From<F> for Rule<T>
	where F: Fn(&T) -> ValidationResult + 'static,
	      T: ? Sized {
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