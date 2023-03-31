use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, Responder, web};
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

        App::new().service(index)
    }).bind(("0.0.0.0", 8080))?.run().await
}