#[derive(Debug)]
pub struct UserProfile {
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub photo: String,
    pub user_id:uuid::Uuid,
}

