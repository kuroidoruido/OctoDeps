use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub groups: Vec<Group>,
    pub apps: Vec<App>,
}

#[derive(Clone, Debug)]
pub struct OctoDepsState {
    pub groups: Vec<Group>,
    pub apps: Vec<App>,
    pub last_updated_on: Option<DateTime<Utc>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub name: String,
    pub group: String,
    // use for config file
    pub asset_version_urls: Vec<String>,
    // use for template
    pub asset_infos: Option<Vec<AssetInfos>>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfos {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<Dependency>,
    pub dev_dependencies: Vec<Dependency>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub highlight: bool,
    pub dependency_outdated: DependencyOutdated,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DependencyOutdated {
    pub latest: Option<String>,
    pub wanted: Option<String>,
}

#[derive(serde::Serialize)]
pub struct TemplateContext {
    pub last_updated_on: String,
    pub groups_map: HashMap<String, Group>,
    pub groups: Vec<Group>,
    pub apps: Vec<App>,
}
