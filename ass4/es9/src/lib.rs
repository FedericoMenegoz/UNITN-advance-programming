pub mod operating_system;
use crate::operating_system as os;
use crate::operating_system::user::User;
#[allow(dead_code)]
fn sudo_change_permission( user: &mut User, string: String, permission: os::permission::Permission, value: bool)  -> Result<(), &str> {
    let action = user.actions
        .iter_mut()
        .find(|s| *s.action == string);
    match action {
        Some(a) => {
            let val = a.permission
                .get_mut(&permission)
                .expect(format!("{:?} key always present!", permission).as_str());
            *val = value;
            Ok(())
        },
        None => Err("Action not found for this user.")
    }
}
/** Sudo change permission if action not present then it create the action with the corresponding
value to that permission leaving the others permission to false*/
/*fn sudo_change_permission(
    user: &mut os::user::User,
    string: String,
    permission: os::Permission,
    value: bool) {
    let (read, write, execute) = match permission {
        os::Permission::READ => (value, false, false),
        os::Permission::WRITE => (false, value, false),
        os::Permission::EXECUTE => (false, false, value)
    };
    let action = os::Actions::new(string, read, write, execute);
    user.actions.push(action);
}*/
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::{operating_system::user::User, sudo_change_permission};
    use crate::operating_system::{Auth,permission::Permission, role::Role, actions::Actions};

    fn create_user_action_for_testing() -> Vec<Actions> {
        vec![
            Actions {
                action: "rw".to_owned(),
                permission: HashMap::from([
                    (Permission::READ, true),
                    (Permission::WRITE, true),
                    (Permission::EXECUTE, false),
                ])
            },
            Actions {
                action: "w".to_owned(),
                permission: HashMap::from([
                    (Permission::WRITE, true),
                    (Permission::READ, false),
                    (Permission::EXECUTE, false),
                ])
            },
            Actions {
                action: "r".to_owned(),
                permission: HashMap::from([
                    (Permission::WRITE, false),
                    (Permission::READ, true),
                    (Permission::EXECUTE, false),
                ])
            },
            Actions {
                action: "rwx".to_owned(),
                permission: HashMap::from([
                    (Permission::WRITE, true),
                    (Permission::READ, true),
                    (Permission::EXECUTE, true),
                ])
            }
        ]
    }
    fn create_admin_test() -> User {
        User {
            name: "Admin".to_owned(),
            role: Role::ADMIN,
            actions: create_user_action_for_testing()
        }
    }

    fn create_user_test_user() -> User {
        User {
            name: "User".to_owned(),
            role: Role::USER,
            actions: create_user_action_for_testing()
        }
    }
    #[test]
    fn check_permission_action_not_present() {
        let admin = create_admin_test();
        assert_eq!(false, admin.check_permission("rwee", &Permission::EXECUTE));
    }

    #[test]
    fn check_permission_action_present_permission_true() {
        let admin = create_admin_test();
        assert_eq!(true, admin.check_permission("rw", &Permission::READ));
    }

    #[test]
    fn check_permission_action_present_permission_false() {
        let admin = create_admin_test();
        assert_eq!(false, admin.check_permission("rw", &Permission::EXECUTE));
    }

    #[test]
    fn can_write_action_present_false() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_write("r"));
    }

    #[test]
    fn can_write_action_present_true() {
        let admin = create_admin_test();
        assert_eq!(true, admin.can_write("rw"));
    }

    #[test]
    fn can_write_action_not_present() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_write("not present"));
    }

    #[test]
    fn can_read_action_present_false() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_read("w"));
    }

    #[test]
    fn can_read_action_present_true() {
        let admin = create_admin_test();
        assert_eq!(true, admin.can_read("rw"));
    }

    #[test]
    fn can_read_action_not_present() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_read("not present"));
    }

    #[test]
    fn can_execute_action_present_false() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_read("w"));
    }

    #[test]
    fn can_execute_action_present_true() {
        let admin = create_admin_test();
        assert_eq!(true, admin.can_read("rwx"));
    }

    #[test]
    fn can_execute_action_not_present() {
        let admin = create_admin_test();
        assert_eq!(false, admin.can_read("not present"));
    }

    #[test]
    fn impl_default_for_action() {
        let mut permission = HashMap::new();
        permission.insert(Permission::READ, false);
        permission.insert(Permission::WRITE, false);
        permission.insert(Permission::EXECUTE, false);
        let action = "".to_owned();
        assert_eq!(Actions{ action, permission }, Actions::default());
    }

    #[test]
    fn impl_new_for_action() {
        let action = "rw".to_owned();
        let (read, write, execute) = (true, true, false);
        let mut permission = HashMap::new();
        permission.insert(Permission::READ, true);
        permission.insert(Permission::WRITE, true);
        permission.insert(Permission::EXECUTE, false);
        assert_eq!(Actions{ action, permission }, Actions::new("rw".to_string(), read, write, execute));
    }

    #[test]
    fn impl_default_for_user() {
        let name = "Guest".to_owned();
        let role = Role::GUEST;
        let actions = Vec::new();

        assert_eq!(User { name, role, actions }, User::default());
    }

    #[test]
    fn admin_can_stay_admin() {
        let mut admin = create_admin_test();
        assert_eq!(Ok(()), admin.change_role(Role::ADMIN));
    }
    #[test]
    fn admin_can_change_to_user() {
        let mut admin = create_admin_test();
        assert_eq!(Ok(()), admin.change_role(Role::USER));
    }

    #[test]
    fn admin_can_change_to_guest() {
        let mut admin = create_admin_test();
        assert_eq!(Ok(()), admin.change_role(Role::GUEST));
    }

    #[test]
    fn user_cannot_change_to_admin() {
        let mut user = create_user_test_user();
        assert_eq!(Err("User can not change role to admin.".to_owned()), user.change_role(Role::ADMIN));
    }

    #[test]
    fn user_can_stay_user() {
        let mut user = create_user_test_user();
        assert_eq!(Ok(()), user.change_role(Role::USER));
    }

    #[test]
    fn user_can_change_to_guest() {
        let mut user = create_user_test_user();
        assert_eq!(Ok(()), user.change_role(Role::GUEST));
    }

    #[test]
    fn guest_can_not_change_to_admin() {
        let mut guest = User::default();
        assert_eq!(Err("Guest can not change role to admin.".to_owned()), guest.change_role(Role::ADMIN));
    }

    #[test]
    fn guest_can_not_change_to_user() {
        let mut guest = User::default();
        assert_eq!(Err("Guest can not change role to user.".to_owned()), guest.change_role(Role::USER));
    }

    #[test]
    fn guest_can_stay_guest() {
        let mut guest = User::default();
        assert_eq!(Ok(()), guest.change_role(Role::GUEST));
    }

    fn create_guest_for_testing_sudo() -> User {
        User {
            name: "test sudo".to_string(),
            role: Role::GUEST,
            actions: vec! [Actions{
                action: "no permission".to_string(),
                permission: HashMap::from([
                    (Permission::READ, false),
                    (Permission::WRITE, false),
                    (Permission::EXECUTE, false)
                ])
            }]
        }
    }

    #[test]
    fn sudo_change_permission_write_true() {
        let mut user = create_guest_for_testing_sudo();
        sudo_change_permission(&mut user, "no permission".to_string(), Permission::WRITE, true)
            .expect("\"no permission\" action should be present in test user");
        assert_eq!(*user.actions[0]
            .permission
            .get(&Permission::WRITE)
            .expect("Key WRITE always present in action."), true)
    }

}