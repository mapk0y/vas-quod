extern crate getopts;
use getopts::Options;
use std::env;

mod runtime;
mod cgroup;
mod filesystem;
mod mount;
mod namespace;

fn run(rootfs: Option<String>, command_string: Option<String>){
	let rootfs = rootfs.unwrap(); // unwrap() は Ok なら値を、Err なら panic
	let command_string = command_string.unwrap();

	let child_command_buffer = command_string.split(" "); // " " で区切られたイテレータの生成 
	let mut child_command_vector = child_command_buffer.collect::<Vec<&str>>(); // Vec は可変長配列, collect で Vec へ入れてる
	let command = child_command_vector[0];
	child_command_vector.drain(0..1); // child_command_vector 配列の 0 から 1 取り出して削除

	runtime::run_container(&rootfs, command, child_command_vector);
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} vas-quod [options]", program);
	print!("{}", opts.usage(&brief));
}


fn main() {
	
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optopt("r", "rootfs", "Path to root file-system eg. --rootfs /home/alpinefs", "path");
	opts.optopt("c", "command", "Command to be executed eg. --command `curl http://google.com`", "command");
	opts.optflag("h", "help", "print this help menu");
	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m }
		Err(_f) => {
			println!("Error: Unrecognzied Options");
			print_usage(&program, opts);
			return
		}
	};
	if matches.opt_present("h") || !matches.opt_present("r") || !matches.opt_present("c") {
		print_usage(&program, opts);
		return;
	}

	let rootfs = matches.opt_str("r");
	let command_string = matches.opt_str("c");
	run(rootfs, command_string);
}
