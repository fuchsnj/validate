use super::{Rule, ValidationError};
use super::bound::Bound;
use regex::Regex;
use std::fmt::Display;

pub fn whitelist_chars(whitelist: &str) -> Rule<str> {
	let whitelist = whitelist.to_owned();
	Rule::from(move |input: &str| {
		for c in input.chars() {
			if !whitelist.contains(c) {
				return Err(ValidationError(format!("cannot contain the character '{}'", c)));
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
			false => Err(ValidationError(format!("must be {}", description)))
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
				Err(ValidationError(format!("must be < {}", end)))
			}
		}),
		Bound::Range(start, end) => Rule::from(move |input: &T| {
			if input >= &start && input < &end {
				Ok(())
			} else {
				Err(ValidationError(format!("must be >= {} and < {}", start, end)))
			}
		}),
		Bound::Exact(value) => Rule::from(move |input: &T| {
			if input == &value {
				Ok(())
			} else {
				Err(ValidationError(format!("must equal {}", value)))
			}
		}),
		Bound::RangeFrom(start) => Rule::from(move |input: &T| {
			if input >= &start {
				Ok(())
			} else {
				Err(ValidationError(format!("must be >= {}", start)))
			}
		})
	}
}

#[test]
fn validate_length() {
	assert!(Rule::map("length", str::len, bound(1..5)).validate("1234").is_ok());
	assert!(Rule::map("length", str::len, bound(1..5)).validate("12345").is_err());
	assert_eq!(Rule::map("length", str::len, bound(2)).validate("1").unwrap_err().0, "length must equal 2");
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
	assert!(Rule::nest("x", TestData::get_x, bound(1)).validate(&data).is_ok());
	assert_eq!(Rule::nest("x", TestData::get_x, bound(2)).validate(&data).unwrap_err().0, "x must equal 2".to_owned());
	assert!(Rule::nest("y", |d: &TestData| &d.y,
	                   Rule::map("length", String::len, bound(4) + bound(3..6))).validate(&data).is_ok()
	);
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