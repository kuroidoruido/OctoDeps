#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use structopt::StructOpt;

mod http_server;
mod models;
mod octodeps_opt;
mod state;

fn main() {
    let opt = octodeps_opt::OctodepsOpt::from_args();

    let groups = state::get_groups();
    let apps = state::get_apps();

    http_server::start(groups, apps);
}
