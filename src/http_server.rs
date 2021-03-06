use std::collections::HashMap;
use chrono::Utc;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use crate::models;

#[get("/")]
fn index(groups_state: State<Vec<models::Group>>, apps_state: State<Vec<models::App>>) -> Template {
    let groups = groups_state.to_vec();
    let mut groups_map: HashMap<String, models::Group> = HashMap::new();
    for group in groups.iter() {
        groups_map.insert(group.id.clone(), group.clone());
    }
    let apps = apps_state.to_vec().clone();
    let last_updated_on = Utc::now().format("%F %X").to_string();
    let context = models::TemplateContext { last_updated_on, groups, groups_map, apps };
    Template::render("index", &context)
}
#[get("/apps")]
fn get_apps_json(app_infos: State<Vec<models::App>>) -> Json<Vec<models::App>> {
    Json(app_infos.to_vec())
}
#[get("/groups")]
fn get_groups_json(groups: State<Vec<models::Group>>) -> Json<Vec<models::Group>> {
    Json(groups.to_vec())
}


pub fn start(groups: Vec<models::Group>, apps: Vec<models::App>) {
    rocket::ignite()
        .mount("/", routes![index, get_apps_json, get_groups_json])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(Template::fairing())
        .manage(apps)
        .manage(groups)
        .launch();
}