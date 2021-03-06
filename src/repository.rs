pub trait Repository {
    fn get_user(&self, user_id: Uuid::uuid) -> Result<User, String> {}
}
