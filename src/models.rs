use schema::users;

#[derive(Queryable)]
pub struct User {
  pub id: i32,
  pub hash: String,
  pub salt: String,
  pub email: String,
  pub phone: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub phone: &'a str,
    pub hash: &'a str,
}