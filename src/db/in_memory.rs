use std::result::Result;
use std::collections::HashMap;

use super::db::DB;
use super::super::model::resource::DataResource;

use std::sync::{Arc, Mutex};
use std::cell::RefCell;

// warning: not thread safe
pub struct InMemoryDB {
    pub map: Arc<Mutex<RefCell<HashMap<String, DataResource>>>>
}

impl DB for InMemoryDB {
    fn save(&self, resource: DataResource) -> Result<(), String> {
        self.map.lock().unwrap().borrow_mut().insert(resource.id.clone(), resource);
        return Result::Ok(());
    }

    fn retrieve(&self, id: &String) -> Result<Option<DataResource>, String> {
        
        return match self.map.lock().unwrap().borrow_mut().get(id) {
            Some(x) => Result::Ok(Some(x.clone())),
            None => Result::Err("No entry found".into())
        };
    }
}
