use crate::operating_system::actions::Actions;
use crate::operating_system::Auth;
use crate::operating_system::permission::Permission;
use crate::operating_system::role::Role;

#[derive(PartialEq, Debug)]
pub struct User {
    pub name: String,
    pub role: Role,
    pub actions: Vec<Actions>
}

impl Auth for User {
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool {
        match self.actions.iter().find(|&x| x.action == action) {
            Some(a) => *a.permission
                .get(permission_type)
                .unwrap_or_else(|| &false),
            None => false
        }
    }

    fn can_write(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::WRITE)
    }

    fn can_read(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::READ)
    }

    fn can_execute(&self, string: &str) -> bool {
        self.check_permission(string, &Permission::READ)
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "Guest".to_string(),
            role: Role::GUEST,
            actions: vec![]
        }
    }
}

impl User {
    pub fn change_role(&mut self, role: Role) -> Result<(), String> {
        match (&mut self.role, role) {
            (a, b) => {
                if *a == Role::GUEST && b != Role::GUEST {
                    Err(format!("Guest can not change role to {}.", b))
                } else if *a == Role::USER && b == Role::ADMIN {
                    Err(format!("User can not change role to admin."))
                } else {
                    *a = b;
                    Ok(())
                }
            }
        }
    }
}