use std::result::Result;
use std::collections::HashMap;

use super::db::DB;
use super::super::model::resource::DataResource;

// warning: not thread safe
pub struct InMemoryDB {
    map: HashMap<String, DataResource>
}

impl DB for InMemoryDB {
    fn save(&self, id: String, resource: DataResource) -> Result<(), String> {
        self.map.insert(id, resource);
        return Result::Ok(());
    }

    fn retrieve(&self, id: String) -> Result<Option<&DataResource>, String> {
        return Result::Ok(self.map.get(&id));
    }
}
