use std::result::Result;
use std::collections::HashMap;

use super::db::DB;
use super::super::model::resource::DataResource;

// warning: not thread safe
pub struct InMemoryDB {
    pub map: HashMap<String, DataResource>
}

impl DB for InMemoryDB {
    fn save(&mut self, resource: DataResource) -> Result<(), String> {
        self.map.insert(resource.id.clone(), resource);
        return Result::Ok(());
    }

    fn retrieve(self, id: &String) -> Result<Option<DataResource>, String> {
        let referenced = self.map.get(id);
        
        return match referenced {
            Some(x) => Result::Ok(Some(x.clone())),
            None => Result::Err("No entry found".into())
        };
    }
}
