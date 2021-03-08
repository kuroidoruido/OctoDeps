use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use chrono::Utc;
use job_scheduler::{Job,JobScheduler,Schedule};

use crate::models::OctoDepsState;

pub fn grab_state_periodically(state: &'static RwLock<OctoDepsState>, config_path: String) {
    thread::spawn(move|| {
        let mut job_scheduler = JobScheduler::new();
        let task = || {
            println!("load config {:?}", config_path);
            let mut rw_state = state.write().unwrap();
            rw_state.last_updated_on = Some(Utc::now());
            println!("Get executed {:?}", rw_state.last_updated_on);
            // let config = config_reader::read_config(config_path);
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