use std::fs::File;
use std::io::BufReader;

pub fn open_input(file_path: &str) -> std::io::BufReader<std::fs::File> {
	let file = match File::open(file_path) {
		Err(why) => panic!("couldn't open, {}", why),
		Ok(file) => file,
	};
	BufReader::new(file)
}
