#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod http_server;
mod models;
mod state;

fn main() {
    let groups = state::get_groups();
    let apps = state::get_apps();

    http_server::start(groups, apps);
}