use serde_derive::Serialize;
use super::schema::teachers;
#[derive(Debug, Serialize, Queryable)]
pub struct Teachers {
    pub id: String,
    pub name: String,
    pub email: String,
}
#[derive(Insertable)]
#[table_name = "teachers"]
pub struct NewTeacher<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub email: &'a str,
}
