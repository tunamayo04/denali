use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::repositories::schema::vendors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Vendor {
    pub id: i32,
    pub name: String,
}