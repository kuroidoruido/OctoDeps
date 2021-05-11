use crate::models::OctoDepsState;
use std::sync::RwLock;

lazy_static! {
    pub static ref STATE: RwLock<OctoDepsState> = RwLock::new(OctoDepsState {
        groups: Vec::new(),
        apps: Vec::new(),
        last_updated_on: None,
    });
}
