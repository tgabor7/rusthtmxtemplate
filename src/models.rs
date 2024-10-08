use diesel::prelude::*;
use crate::schema::posts;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}