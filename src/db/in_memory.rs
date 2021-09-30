use std::result::Result;
use std::collections::HashMap;

use super::db::DB;
use super::super::model::resource::DataResource;

use std::sync::{Mutex};
use std::cell::RefCell;

// warning: not thread safe
pub struct InMemoryDB {
    pub map: Mutex<RefCell<HashMap<String, DataResource>>>
}

impl InMemoryDB {
    pub fn new() -> InMemoryDB {
        InMemoryDB {
            map: Mutex::new(RefCell::new(HashMap::new()))
        }
    }
}

impl DB for InMemoryDB {
    fn save(&self, resource: DataResource) -> Result<(), String> {
        let guard = self.map.lock().unwrap();
        let mut hmap = guard.borrow_mut();

        hmap.insert(resource.id.clone(), resource);
        return Result::Ok(());
    }

    fn retrieve(&self, id: &String) -> Result<Option<DataResource>, String> {
        let guard = self.map.lock().unwrap();
        let hmap = guard.borrow_mut();

        return match hmap.get(id) {
            Some(x) => Result::Ok(Some(x.clone())),
            None => Result::Err("No entry found".into())
        };
    }
}
