use crate::models::{App,AssetInfos,Dependency,DependencyOutdated,Group};

pub fn get_apps() -> Vec<App> {
    vec![
        App {
            name: String::from("OctoDeps"),
            group: String::from("default"),
            asset_version_urls: vec![],
            asset_infos: vec![
                AssetInfos {
                    name: String::from("OctoDeps (Rust)"),
                    version: String::from("0.1.0"),
                    dependencies: vec![
                        Dependency {
                            name: String::from("rocket"),
                            version: String::from("0.4.7"),
                            highlight: true,
                            dependency_outdated: DependencyOutdated {
                                latest: None,
                                wanted: Some(String::from("0.4.8")),
                            },
                        },
                        Dependency {
                            name: String::from("rocket_contrib"),
                            version: String::from("0.4.7"),
                            highlight: true,
                            dependency_outdated: DependencyOutdated {
                                wanted: Some(String::from("0.4.7")),
                                latest: Some(String::from("0.5.0-dev")),
                            },
                        },
                        Dependency {
                            name: String::from("serde"),
                            version: String::from("1.0.123"),
                            highlight: false,
                            dependency_outdated: DependencyOutdated { latest: None, wanted: None },
                        }
                    ],
                    dev_dependencies: vec![],
                }
            ]
        }
    ]
}

pub fn get_groups() -> Vec<Group> {
    vec![
        Group {
            id: String::from("default"),
            name: String::from("Default"),
            // color: String::from("#757475")
            color: String::from("#4c7c2a")
        }
    ]
}