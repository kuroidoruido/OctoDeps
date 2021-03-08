#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

// use std::sync::RwLock;
use structopt::StructOpt;

mod config_reader;
mod http_client;
mod http_server;
mod models;
mod octodeps_opt;
mod scheduler;
mod state;

fn main() {
    let opt = octodeps_opt::OctodepsOpt::from_args();
    scheduler::grab_state_periodically(&state::STATE, opt.config);
    http_server::start(&state::STATE);
}