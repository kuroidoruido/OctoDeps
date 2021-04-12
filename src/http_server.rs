use std::collections::HashMap;
use std::sync::RwLock;
use chrono::{DateTime,Utc,TimeZone};
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::models::{App,Group,OctoDepsState,TemplateContext};

#[get("/")]
fn index(state: State<&RwLock<OctoDepsState>>) -> Template {
    let read_state = state.read().unwrap();
    let apps = read_state.apps.clone();
    let groups = read_state.groups.clone();
    let last_updated_on_date: DateTime<Utc> = match read_state.last_updated_on {
        None => Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0),
        Some(d) => d
    };
    let last_updated_on = last_updated_on_date.format("%F %X").to_string();
    let mut groups_map: HashMap<String, Group> = HashMap::new();
    for group in groups.iter() {
        groups_map.insert(group.id.clone(), group.clone());
    }
    let context = TemplateContext { last_updated_on, groups_map, groups, apps };
    Template::render("index", &context)
}

#[get("/apps")]
fn get_apps_json(state: State<&RwLock<OctoDepsState>>) -> Json<Vec<App>> {
    let read_state = state.read().unwrap();
    Json(read_state.apps.clone())
}

#[get("/groups")]
fn get_groups_json(state: State<&RwLock<OctoDepsState>>) -> Json<Vec<Group>> {
    let read_state = state.read().unwrap();
    Json(read_state.groups.clone())
}

pub fn start(state: &'static RwLock<OctoDepsState>) {
    rocket::ignite()
        .mount("/", routes![index, get_apps_json, get_groups_json])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(Template::fairing())
        .manage(state)
        .launch();
}