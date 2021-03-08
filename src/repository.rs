use crate::user::User;
use uuid::Uuid;
pub trait Repository {
    fn get_user(&self, user_id: Uuid) -> Result<User, String>;
}
pub struct MemoryReposiory {
    users: Vec<User>,
}
impl Default for MemoryReposiory {
    fn default() -> Self {
        Self {
            users: vec![User::new("amanda".to_string(), (1991, 11, 9))],
        }
    }
}

impl Repository for MemoryReposiory {
    fn get_user(&self, user_id: uuid::Uuid) -> Result<User, String> {
        self.users
            .iter()
            .find(|u| u.id == user_id)
            .map(|u| u.clone())
            .ok_or_else(|| "invalid UUID".to_string())
    }
}
