use std::path::Path;

///A function used to convert the first letter of the string to uppercase, to it can be parsed and displayed correctly
pub fn uppercase_converter(s: &mut String) {
	let mut v: Vec<char> = s.chars().collect();
	v[0] = v[0].to_uppercase().nth(0).unwrap();
	let s2: String = v.into_iter().collect();
	*s = s2.clone();
}

///A function to check if the config.txt file that holds the url exists
pub fn file_check() -> bool {
	let mut _rs: bool = true;

	_rs = Path::new("./config.txt").exists();

	if _rs == true {
		println!("File already exists");
	} else {
		println!("File does not exist, creating file");
	}
	_rs
}
