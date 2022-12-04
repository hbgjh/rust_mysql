//filename:mysql_helper.rs
use mysql::*;
use mysql::prelude::*;

pub struct User
{
   pub id: i32,
   pub name: String,
   pub age: i32,
}

pub fn conn_mysql()->PooledConn{
    //设置连接字符串 localhost 注意端口:3306
    let _url="mysql://root:Ca2OOf3U6aBV7Nz3@localhost:3306/wz96333releasedb";
    let pool=Pool::new(_url).unwrap();
    //连接数据库
    let conn=pool.get_conn().unwrap();
    return conn;
}

pub fn select(sqlstr: &str)->User{
        //连接数据库
        let mut conn=conn_mysql();
        let mut user=User{id:0,name:String::from(""), age:0};
        conn.query_iter(sqlstr).unwrap()
        .for_each(|row|{
            let r:(i32,String,i32)=from_row(row.unwrap());
            user=User{id:r.0,name:String::from(r.1), age:r.2};
        });

        return user;
    }
    
pub fn delete(id: i32){
        //连接数据库
        let mut conn=conn_mysql();
        let stmt = conn.prep("delete from user where id=:id").unwrap();
    
        conn.exec_drop(&stmt, params! {
            "id" => id,
        }).unwrap();
    }
    
pub fn insert(user:User){
        //连接数据库
        let mut conn=conn_mysql();
        conn.exec_drop(
            "INSERT INTO user (name,age) VALUES (:name, :age)",
            params! {
                "name" => user.name,
                "age" => user.age,
        }).unwrap();
    }
    
pub fn update(user: User){
        //连接数据库
        let mut conn=conn_mysql();

        let stmt = conn.prep("update user set name=:name, age=:age where id=:id")
        .unwrap();
       
        conn.exec_drop(&stmt, params! {
           "name" => user.name,
           "age" => user.age,
           "id" => user.id,
        }).unwrap();
    }
