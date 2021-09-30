use std::result::Result;
use std::option::Option;

use super::super::model::resource::DataResource;

pub trait DB: Send + Sync {
    // fill out your own `Err` types
    fn save(&mut self, resource: DataResource) -> Result<(), String>;
    fn retrieve(self, id: &String) -> Result<Option<DataResource>, String>;
}
