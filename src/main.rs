#![feature(proc_macro_hygiene, decl_macro)]
use std::collections::HashMap;
use chrono::Utc;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

mod models;

#[macro_use] extern crate rocket;

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

fn get_apps() -> Vec<models::App> {
    vec![
        models::App {
            name: String::from("OctoDeps"),
            group: String::from("default"),
            asset_version_urls: vec![],
            asset_infos: vec![
                models::AssetInfos {
                    name: String::from("OctoDeps (Rust)"),
                    version: String::from("0.1.0"),
                    dependencies: vec![
                        models::Dependency {
                            name: String::from("rocket"),
                            version: String::from("0.4.7"),
                            highlight: true,
                            dependency_outdated: models::DependencyOutdated {
                                latest: None,
                                wanted: Some(String::from("0.4.8")),
                            },
                        },
                        models::Dependency {
                            name: String::from("rocket_contrib"),
                            version: String::from("0.4.7"),
                            highlight: true,
                            dependency_outdated: models::DependencyOutdated {
                                wanted: Some(String::from("0.4.7")),
                                latest: Some(String::from("0.5.0-dev")),
                            },
                        },
                        models::Dependency {
                            name: String::from("serde"),
                            version: String::from("1.0.123"),
                            highlight: false,
                            dependency_outdated: models::DependencyOutdated { latest: None, wanted: None },
                        }
                    ],
                    dev_dependencies: vec![],
                }
            ]
        }
    ]
}

fn get_groups() -> Vec<models::Group> {
    vec![
        models::Group {
            id: String::from("default"),
            name: String::from("Default"),
            // color: String::from("#757475")
            color: String::from("#4c7c2a")
        }
    ]
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_apps_json, get_groups_json])
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .attach(Template::fairing())
        .manage(get_apps())
        .manage(get_groups())
        .launch();
}