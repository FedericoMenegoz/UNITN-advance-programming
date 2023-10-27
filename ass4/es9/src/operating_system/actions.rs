use std::collections::HashMap;
use crate::os::permission::Permission;

#[derive(PartialEq, Debug)]
pub struct Actions {
    pub action: String,
    pub permission: HashMap<Permission, bool>
}
impl Default for Actions {
    fn default() -> Self {
        Actions {
            action: String::new(),
            permission: HashMap::from([
                (Permission::EXECUTE, false),
                (Permission::WRITE, false),
                (Permission::READ, false)
            ])}
    }
}
impl Actions {
    pub fn new(action: String, read: bool, write: bool, execute: bool) -> Self{
        Actions {
            action,
            permission: HashMap::from([
                (Permission::READ, read),
                (Permission::WRITE, write),
                (Permission::EXECUTE, execute)
            ])}
    }
}