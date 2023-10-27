use std::fmt::{Display, Formatter};
#[derive(PartialEq, Eq, Debug)]
pub enum Role {
    GUEST,
    USER,
    ADMIN
}
impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let role = match self {
            Role::GUEST => "guest",
            Role::USER => "user",
            Role::ADMIN => "admin"
        };
        write!(f, "{}", role)
    }
}