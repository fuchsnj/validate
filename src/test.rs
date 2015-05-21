use rules::*;

#[test]
fn validate_range() {
	assert!(range(1,7).validate(&1).is_ok());
	assert!(range(1,7).validate(&2).is_ok());
	assert!(range(1,7).validate(&6).is_ok());
	assert!(range(1,7).validate(&7).is_ok());

	assert!(range(1,7).validate(&0).is_err());
	assert!(range(1,7).validate(&8).is_err());
}

#[test]
fn convert_range() {
	assert!(range(1,7).convert(&"1").is_ok());
	assert!(range(1,7).convert(&"2").is_ok());
	assert!(range(1,7).convert(&"6").is_ok());
	assert!(range(1,7).convert(&"7").is_ok());

	assert!(range(1,7).convert(&"0").is_err());
	assert!(range(1,7).convert(&"8").is_err());
	assert!(range(1,7).convert(&"a").is_err());
}

#[test]
fn validate_regex(){
	assert!(match_regex("h..h").validate(&"haah".to_string()).is_ok());
	assert!(match_regex("h..h").validate(&"aargh".to_string()).is_err());
}

#[test]
fn validate_email() {
	assert!(email().convert(&"test@domain.com").is_ok());
	assert!(email().convert(&"test@domaincom").is_err());
	assert!(email().convert(&"testdomain.com").is_err());
	assert!(email().convert(&"@domaincom").is_err());
	assert!(email().convert(&"@domain.com").is_err());
	assert!(email().convert(&"test@").is_err());
	assert!(email().convert(&"test").is_err());

	assert!(email().convert(&"test123+123@domain.com").is_ok());
	assert!(email().convert(&"test123.4@domain.com").is_ok());
	assert!(email().convert(&"test1.2.3.4@domain.com").is_ok());
}