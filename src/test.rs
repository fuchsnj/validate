use Schema;

#[test]
fn validate_regex(){
	assert!(Schema::new().match_regex("h..h").validate(&"haah").is_ok());
	assert!(Schema::new().match_regex("h..h").validate(&"aargh").is_err());
}


#[test]
fn validate_range() {
	assert!(Schema::new().range(1..2).validate(&1).is_ok());
	assert!(Schema::new().range(1..).validate(&1).is_ok());
	assert!(Schema::new().range(1..).validate(&2).is_ok());
	assert!(Schema::new().range(..2).validate(&1).is_ok());
	assert!(Schema::new().range(..).validate(&1).is_ok());
	
	assert!(Schema::new().range(1..2).validate(&0).is_err());
	assert!(Schema::new().range(1..2).validate(&2).is_err());
	assert!(Schema::new().range(1..).validate(&0).is_err());
	assert!(Schema::new().range(..2).validate(&2).is_err());
}


#[test]
fn validate_length(){
	assert!(Schema::new().length(4).validate(&"1234").is_ok());
	assert!(Schema::new().length(1..3).validate(&"1").is_ok());
	assert!(Schema::new().length(1..3).validate(&"12").is_ok());
	assert!(Schema::new().length(..3).validate(&"12").is_ok());
	assert!(Schema::new().length(2..).validate(&"12").is_ok());
	assert!(Schema::new().length(..).validate(&"12").is_ok());
	
	assert!(Schema::new().length(4).validate(&"123").is_err());
	assert!(Schema::new().length(1..3).validate(&"").is_err());
	assert!(Schema::new().length(1..3).validate(&"123").is_err());
	assert!(Schema::new().length(..3).validate(&"123").is_err());
	assert!(Schema::new().length(2..).validate(&"1").is_err());
}

#[test]
fn validate_email() {
	assert!(Schema::new().email().validate(&"test@domain.com").is_ok());
	assert!(Schema::new().email().validate(&"test123+123@domain.com").is_ok());
	assert!(Schema::new().email().validate(&"test123.4@domain.com").is_ok());
	assert!(Schema::new().email().validate(&"test1.2.3.4@domain.com").is_ok());
	
	assert!(Schema::new().email().validate(&"test@domaincom").is_err());
	assert!(Schema::new().email().validate(&"testdomain.com").is_err());
	assert!(Schema::new().email().validate(&"@domaincom").is_err());
	assert!(Schema::new().email().validate(&"@domain.com").is_err());
	assert!(Schema::new().email().validate(&"test@").is_err());
	assert!(Schema::new().email().validate(&"test").is_err());
}

#[test]
fn validate_complex(){
	let schema = Schema::new().email().length(1..100);
	assert!(schema.validate(&"test@domain.com").is_ok());
	assert!(schema.validate(&"notvalidemail").is_err());
}
