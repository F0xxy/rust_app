use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub custom_data: CustomData,
    pub birth_date: NaiveDate,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (y, m, d) = birth_date_ymd;
        Self {
            id: Uuid::parse_str("87d512c3-9141-46aa-a642-3d2673fa58ca").unwrap(),
            name,
            birth_date: NaiveDate::from_ymd(y, m, d),
            custom_data: CustomData { random: 1 },
            created_at: Some(Utc::now()),
            updated_at: None,
            deleted_at: None,
        }
    }
}
