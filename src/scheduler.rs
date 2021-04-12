use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use chrono::Utc;
use job_scheduler::{Job,JobScheduler,Schedule};

use crate::config_reader;
use crate::http_client::get_asset_info;
use crate::models::OctoDepsState;

pub fn grab_state_periodically(state: &'static RwLock<OctoDepsState>, config_path: String) {
    thread::spawn(move|| {
        let mut job_scheduler = JobScheduler::new();
        let task = || {
            println!("load config {:?}", config_path);
            let config_result = config_reader::read_config(config_path.clone());
            if let Ok(mut config) = config_result {
                for app in config.apps.iter_mut() {
                    let mut asset_infos = Vec::new();
                    for asset_url in app.asset_version_urls.iter() {
                        println!("load asset infos {:?}", asset_url);
                        let asset_infos_result = get_asset_info(asset_url.clone());
                        if let Ok(asset_infos_result_ok) = asset_infos_result {
                            for asset_info in asset_infos_result_ok {
                                asset_infos.push(asset_info);
                            }
                        } else {
                            println!("fail to load {:?} {:?}", asset_url, asset_infos_result.err().unwrap());
                        }
                    }
                    app.asset_infos = Some(asset_infos);
                }
                {
                    let mut rw_state = state.write().unwrap();
                    rw_state.last_updated_on = Some(Utc::now());
                    rw_state.groups = config.groups;
                    rw_state.apps = config.apps;
                    println!("Get executed {:?}", rw_state.last_updated_on);
                }
            }
        };
        // every 10 seconds
        // let schedule: Schedule = "1/10 * * * * * *".parse().unwrap();
        // every minute
        // let schedule: Schedule = "0 * * * * * *".parse().unwrap();
        // every hours
        let schedule: Schedule = "0 0 * * * * *".parse().unwrap();
        let job = Job::new(schedule, task);
        job_scheduler.add(job);
        task();
        loop {
            job_scheduler.tick();
            std::thread::sleep(Duration::from_millis(1000));
        }
    });
}