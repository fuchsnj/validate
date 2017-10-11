use super::{Rule, ValidationResult, Error};
use super::bound::Bound;
use regex::Regex;
use std::fmt::Display;

pub fn whitelist_chars(whitelist: &str) -> Rule<str> {
	let whitelist = whitelist.to_owned();
	Rule::from(move |input: &str| {
		for c in input.chars() {
			if !whitelist.contains(c) {
				return Err(format!("cannot contain the character '{}'", c).into());
			}
		}
		Ok(())
	})
}

pub fn regex(pattern: &str, description: &str) -> Rule<str> {
	let regex = Regex::new(pattern).unwrap();
	let description = description.to_owned();
	Rule::from(move |input: &str| {
		match regex.is_match(input) {
			true => Ok(()),
			false => Err(format!("must be {}", description).into())
		}
	})
}

pub fn email() -> Rule<str> {
	regex(r"^[^@]+@[^@]+\.[^@]+$", "a valid email")
}

pub fn bound<T, R>(bound: R) -> Rule<T>
	where T: Display + PartialOrd + PartialEq + 'static,
	      R: Into<Bound<T>> {
	let bound = bound.into();
	match bound {
		Bound::RangeTo(end) => Rule::from(move |input: &T| {
			if input < &end {
				Ok(())
			} else {
				Err(format!("must be < {}", end).into())
			}
		}),
		Bound::Range(start, end) => Rule::from(move |input: &T| {
			if input >= &start && input < &end {
				Ok(())
			} else {
				Err(format!("must be >= {} and < {}", start, end).into())
			}
		}),
		Bound::Exact(value) => Rule::from(move |input: &T| {
			if input == &value {
				Ok(())
			} else {
				Err(format!("must equal {}", value).into())
			}
		}),
		Bound::RangeFrom(start) => Rule::from(move |input: &T| {
			if input >= &start {
				Ok(())
			} else {
				Err(format!("must be >= {}", start).into())
			}
		})
	}
}

#[test]
fn validate_length() {
	let rule = bound(1..5).name("length").map(str::len);
	assert!(rule.validate("1234").is_ok());
	assert!(rule.validate("12345").is_err());
}


#[test]
fn validate_nest() {
	struct TestData {
		pub x: u32,
		pub y: String
	}

	impl TestData {
		fn get_x(&self) -> &u32 {
			&self.x
		}
	}

	let data = TestData {
		x: 1,
		y: "test".to_owned()
	};

	let rule = Rule::from(move |data: &TestData| {
				bound(0..5).name("x").validate(&data.x)?;
				bound(..5).name("y length").validate(&data.y.len())?;
		Ok(())
	});
	//
	assert!(rule.validate(&data).is_ok());
		assert_eq!(rule.validate(&TestData {
			x: 3,
			y: "too long".to_owned()
		}).unwrap_err().get_message(), "y length must be < 5".to_owned())
}


#[test]
fn test_whitelist() {
	let rule = whitelist_chars("0123456789");
	assert!(rule.validate("12").is_ok());
	assert!(rule.validate("00123456789").is_ok());
	assert!(rule.validate("asdf").is_err());
}

#[test]
fn test_regex() {
	let rule = regex("h.*h", "starts and ends with h");
	assert!(rule.validate(&"haah").is_ok());
	assert!(rule.validate(&"aargh").is_err());
}

#[test]
fn validate_email() {
	let rule = email();
	assert!(rule.validate(&"test@domain.com").is_ok());
	assert!(rule.validate(&"test123+123@domain.com").is_ok());
	assert!(rule.validate(&"test123.4@domain.com").is_ok());
	assert!(rule.validate(&"test1.2.3.4@domain.com").is_ok());

	assert!(rule.validate(&"test@domaincom").is_err());
	assert!(rule.validate(&"testdomain.com").is_err());
	assert!(rule.validate(&"@domaincom").is_err());
	assert!(rule.validate(&"@domain.com").is_err());
	assert!(rule.validate(&"test@").is_err());
	assert!(rule.validate(&"test").is_err());
}

#[test]
fn test_bound() {
	assert!(bound(..4).validate(&3).is_ok());
	assert!(bound(..4).validate(&4).is_err());
	assert!(bound(2..3).validate(&2).is_ok());
	assert!(bound(2..3).validate(&3).is_err());
	assert!(bound(2).validate(&2).is_ok());
	assert!(bound(2).validate(&3).is_err());
	assert!(bound(2..).validate(&2).is_ok());
	assert!(bound(2..).validate(&1).is_err());
}