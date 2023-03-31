use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, post, Responder, web};
use web::Query;
use serde;

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

#[derive(serde::Deserialize,serde::Serialize,Debug)]
struct UserModel {
    user_id: i32,
    user_name: String,
}

#[post("/users")]
async fn users(body: Option<web::Json<UserModel>>)->impl Responder {
    if let Some(user) = body {
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