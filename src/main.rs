use std::io::prelude::*;
extern crate utilities as utils;

use log;

fn main()  {
    env_logger::init();
    log::info!("Starting");
	for line in utils::open_input("./assets/hello_world.txt").lines() {
        log::info!("{}", line.unwrap());
	}
}
