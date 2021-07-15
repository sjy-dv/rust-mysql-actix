use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Result;

const DB_USER:&'static str = "root";
const DB_PASSWORD:&'static str = "1111";
const DB_HOST:&'static str = "localhost";
const DB_PORT: u16 = 3306_u16;
const DB_NAME:&'static str = "rootdb";


//const DB_URL:&'static str = format!("mysql://{}:{}@{}:{}/{}",
  //                      DB_USER, DB_PASSWORD, DB_HOST, DB_PORT, DB_NAME);  
//static const POOL:my = mysql::Pool::new("mysql://root:1111@localhost:3306/rootdb").expect("db connect error!");
/**
 * 
 *  let db_url = format!("mysql://{}:{}@{}:{}/{}",
                        DB_USER, DB_PASSWORD, DB_HOST, DB_PORT, DB_NAME);  
    let pool = mysql::Pool::new(db_url).expect("db connect error!");

    let query = "select * from users";
    let res = pool.prep_exec(query, ()).expect("error");

    for row in res {
        let (username, password):(String, String) = mysql::from_row(row.unwrap());
        println!("{},{}", username, password);
    }
 * 
*/
#[derive(Serialize, Deserialize)]
struct ResponseUser {
    username :String,
    password : String,
}

#[get("/select")]
async fn selectdb() -> impl Responder {
    let db_url = format!("mysql://{}:{}@{}:{}/{}",
                        DB_USER, DB_PASSWORD, DB_HOST, DB_PORT, DB_NAME);  
    let pool = mysql::Pool::new(db_url).expect("db connect error!");

    let query = "select * from users";
    let res = pool.prep_exec(query, ()).expect("error");
    let mut data = Vec::new();
    for row in res {
        let (username, password):(String, String) = mysql::from_row(row.unwrap());
        let format = ResponseUser {
            username : username,
            password : password,
        };
        data.push(format)
    }

    HttpResponse::Ok().json(data)
} 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
   HttpServer::new(|| {
       App::new()
        .service(selectdb)
   })
   .bind("127.0.0.1:8081")?
    .run()
    .await
}