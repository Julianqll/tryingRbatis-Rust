//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;
extern crate rbdc_mysql;
use rbatis::{rbdc::datetime::FastDateTime, Rbatis};
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub email: Option<String>,
}
crud!(Member{});//crud = insert+select_by_column+update_by_column+delete_by_column

impl_select!(Member{select_all_by_id(id:&str,name:&str) => "`where id = #{id} and name = #{name}`"});
impl_select!(Member{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_update!(Member{update_by_name(name:&str) => "`where id = 1`"});
impl_delete!(Member {delete_by_name(name:&str) => "`where name= '2'`"});

#[tokio::main]
async fn main() {
    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let mut rb = Rbatis::new();
    /// connect to database  
    // sqlite 
    //rb.init(SqliteDriver {}, "sqlite://target/sqlite.db").unwrap();
    // mysql 
    rb.init(MysqlDriver{},"mysql://root@localhost:3306/examplerbatis").unwrap();
    // postgresql 
    // rb.init(PgDriver{},"postgres://postgres:123456@localhost:5432/postgres").unwrap();
    // mssql/sqlserver
    // rb.init(MssqlDriver{},"jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test").unwrap();

    let activity = Member {
        id: Some(1.into()),
        name: Some("Julian".into()),
        email: Some("julian@email.com".into()),
    };
    let data = Member::insert(&mut rb, &activity).await;
    println!("insert = {:?}", data);

    // let data = BizActivity::select_all_by_id(&mut rb, "1", "1").await;
    // println!("select_all_by_id = {:?}", data);

    // let data = BizActivity::select_by_id(&mut rb, "1".to_string()).await;
    // println!("select_by_id = {:?}", data);

    // let data = BizActivity::update_by_column(&mut rb, &activity, "id").await;
    // println!("update_by_column = {:?}", data);

    // let data = BizActivity::update_by_name(&mut rb, &activity, "test").await;
    // println!("update_by_name = {:?}", data);

    // let data = BizActivity::delete_by_column(&mut rb, "id", &"2".into()).await;
    // println!("delete_by_column = {:?}", data);

    // let data = BizActivity::delete_by_name(&mut rb, "2").await;
    // println!("delete_by_column = {:?}", data);

}