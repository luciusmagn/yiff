#![feature(proc_macro_hygiene, decl_macro)]

use structopt::StructOpt;
use rocket::{post, routes};
use rocket_contrib::json::Json;

use std::path::PathBuf;
use std::env;
use std::process::Command;

#[derive(Debug, StructOpt)]
struct Cla {
	/// port
	#[structopt(short, long, default_value="8888")]
	port: u16,
}


#[post("/")]
fn base() {
	Command::new("git")
		.arg("pull")
		.spawn()
		.unwrap()
}

fn main() {
	let argv = Cla::from_args();

	env::set_var("ROCKET_PORT", format!("{}", argv.port));

	rocket::ignite()
		.mount("/", routes![base])
		.launch();
}
