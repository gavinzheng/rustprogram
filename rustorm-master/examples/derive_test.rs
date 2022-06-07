extern crate rustorm;
#[macro_use]
extern crate rustorm_derive;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use rustorm::query::TableName;
use rustorm::query::IsTable;
use rustorm::query::ColumnName;
use rustorm::dao::ToValue;
use rustorm::dao::FromValue;
use rustorm::platform::pool;
use rustorm::entity::EntityManager;
use rustorm::query::Filter;
use rustorm::query::Equality;


#[derive(IsDao)]
#[derive(IsTable)]
#[derive(Debug)]
struct Users{
    username: String,
    email: String,
}

fn main() {
    let user = Users{
        username : "ivanceras".to_string(),
        email: "ivanceras@gmail.com".to_string()
    };
    let db = pool::db_with_url("postgres://postgres:p0stgr3s@localhost/mock").unwrap();
    let em = EntityManager::new(&*db);
    let filter = Filter::new("email", Equality::EQ, &"asdsadasd".to_string());
    let users:Vec<Users> = em.get_all_with_filter(&filter).unwrap();
    for i in users{
        println!("{:?}", i);
    }

    let u = em.get_one::<Users>(&filter).unwrap();
    println!("got: {:?}", u);
}
