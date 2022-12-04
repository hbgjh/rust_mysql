use mysql_helper::User;
mod mysql_helper;

fn main ()
{
    //insert
    let user=User{id:1,name:String::from("test"), age:18};
    mysql_helper::insert(user);

    //select
    let sqlstr="SELECT * FROM `user` WHERE ID=2 ORDER BY id DESC".to_string();
    let user:User =mysql_helper::select(&sqlstr);
    println!("User id:{},name:{},age:{},",user.id,user.name,user.age);

    //update
    let user=User{id:2,name:String::from("test11"), age:20};
    mysql_helper::update(user);

    //delete
    mysql_helper::delete(32);
}
