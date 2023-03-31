use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use serde;
use validator::{Validate, ValidationError};
use once_cell::sync::Lazy;
use regex::Regex;

static USERNAME_REG:Lazy<Regex> = Lazy::new(||{
    Regex::new(r"[a-z]{4,20}").unwrap()
});

#[derive(serde::Deserialize,Debug)]
struct IndexParams {
    #[serde(default)]
    id:i32,
    #[serde(default="default_name")]
    name:String
}

fn default_name()->String{
    "guest".to_string()
}

#[derive(serde::Deserialize,serde::Serialize,Debug,Validate)]
struct UserModel {
    #[serde(skip_deserializing)]
    user_id: i32,
    // #[validate(length(min = 4,message="用户名长度必须大于4"))]
    #[validate(regex(path="USERNAME_REG",message="用户名长度必须为4-20且全为小写"))]
    user_name: String,
    #[validate(range(min = 1, max = 100,message="年龄必须介于1-100之间"))]
    user_age:u8,
}

#[post("/users")]
async fn users(body: Option<web::Json<UserModel>>)->impl Responder {
    if let Some(user) = body {
        match user.validate() {
            Ok(_) => return return HttpResponse::Ok().json( user),
            Err(err) => return HttpResponse::Ok().json(err),
        }
        return HttpResponse::Ok().json( user);
    }
    HttpResponse::BadRequest().body("参数格式不对")
}

#[get("/")]
async fn index(req: HttpRequest)->impl Responder {
    // let q:Query<IndexParams> = web::Query::from_query(req.query_string()).unwrap();
    // let q_str = format!("{:?}", q);
    let qs = qstring::QString::from(req.query_string());
    let id = qs.get("id").unwrap_or_default();
    if id == "" {
        return HttpResponse::BadRequest().body("err params:id");
    }
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        App::new().service(index).service(users)
    }).bind(("0.0.0.0", 8080))?.run().await
}