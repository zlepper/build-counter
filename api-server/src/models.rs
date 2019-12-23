use crate::schema::*;
use uuid::Uuid;

#[derive(Queryable, Identifiable, Debug, Eq, PartialEq)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
}

#[derive(Queryable, Associations, Identifiable, Debug, Eq, PartialEq)]
#[belongs_to(Tenant)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub tenant_id: Uuid,
}
