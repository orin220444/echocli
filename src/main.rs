use std::io;
fn main() {
	loop {
		println!("type something");
		let mut usertypings = String::new();
		io::stdin().read_line(&mut usertypings)
        	.expect("Failed to read line");
		let usertypings = usertypings.trim_end();
		if usertypings == "quit" || usertypings == "exit" {
			println!("exiting...");
			break;
		}else {
			println!("you typed: {}", &usertypings);
		}
	}
}
