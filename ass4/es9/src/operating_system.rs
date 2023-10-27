pub mod role;
pub mod permission;
pub mod actions;
pub mod user;

pub trait Auth {
    fn check_permission(&self, action: &str, permission_type: &permission::Permission) -> bool;
    fn can_write(&self, string: &str) -> bool;
    fn can_read(&self, string: &str) -> bool;
    fn can_execute(&self, string: &str) -> bool;
}
