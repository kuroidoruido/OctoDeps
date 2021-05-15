use chrono::{DateTime, TimeZone, Utc};
use rocket::response::content::Html;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use std::collections::HashMap;
use std::sync::RwLock;

use crate::models::{App, Group, OctoDepsState, TemplateContext};
use crate::tera_embeded::{TeraEmbeded, TeraEmbededTemplate};
use crate::tera_tpl;

#[get("/")]
fn index(state: State<&RwLock<OctoDepsState>>) -> Html<Result<String, tera::Error>> {
    let read_state = state.read().unwrap();
    let apps = read_state.apps.clone();
    let groups = read_state.groups.clone();
    let last_updated_on_date: DateTime<Utc> = match read_state.last_updated_on {
        None => Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 0),
        Some(d) => d,
    };
    let last_updated_on = last_updated_on_date.format("%F %X").to_string();
    let mut groups_map: HashMap<String, Group> = HashMap::new();
    for group in groups.iter() {
        groups_map.insert(group.id.clone(), group.clone());
    }
    let context = TemplateContext {
        last_updated_on,
        groups_map,
        groups,
        apps,
    };
    TeraEmbeded::render("index", &context)
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
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(TeraEmbeded::fairing(&[
            tera_tpl!("style"),
            tera_tpl!("header"),
            tera_tpl!("last-update-infos"),
            tera_tpl!("base"),
            tera_tpl!("index"),
        ]))
        .manage(state)
        .launch();
}
