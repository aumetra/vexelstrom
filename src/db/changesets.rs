use crate::db::schema::users;
use diesel::prelude::Insertable;
use uuid::Uuid;

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: Uuid,
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
