use uuid::Uuid;

pub struct User {
    id: Uuid,
    name: String,
    custom_data: CustomData,
    birth_date: NaiveDate,
    created_date: Option<DateTime<Utc>>,
    updated_date: Option<DateTime<Utc>>,
    deleted_date: Option<DateTime<Utc>>,
}
pub struct CustomData {
    random: u32,
}
pub struct NaiveDate {
    date: Option<DateTime<Utc>>,
}
