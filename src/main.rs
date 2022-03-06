use actix_web::HttpRequest;
use std::io;

use actix_files::{Files, NamedFile};
use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{get, web, App, HttpServer, Responder, Result};
use awc::Client;
use serde::{Deserialize, Serialize, Deserializer};

const API_URL: &str = "https://api.unsplash.com/search/photos";

#[derive(Debug, Serialize, Deserialize)]
struct QueryResults {
    // json - results 
    results: Vec<ImgResult>
}

#[derive(Debug, Serialize)]
struct ImgResult {
    // json - results.id 
    id: String,
    // json - results.urls.regular
    url: String,
    // json - results.description
    description: String,
}

impl<'de> Deserialize<'de> for ImgResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        //Top level
        struct Outer {
            id: String,
            #[serde(rename = "urls")]
            // this is coming from results.urls, we only want the "regular" field under urls
            url: Inner,
            // description can be null, so return "" if it is.
            description: Option<String>,
        }

        #[derive(Deserialize)]
        struct Inner {
            regular: String,
        }

        let helper = Outer::deserialize(deserializer)?;
        // put it all together and return as the destination struct
        Ok(ImgResult {
            id: helper.id,
            url: helper.url.regular,
            description: helper.description.unwrap_or_default(),
        })
    }
}


#[derive(Serialize, Deserialize)]
pub struct ImgQuery {
    query: String,
}


#[get("/img")]
async fn get_images(req: HttpRequest, params: web::Query<ImgQuery>) -> Result<impl Responder> {

    let api_key = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let response = Client::new()
        .get(API_URL)
        .insert_header(("Authorization", api_key))
        .query(&params.into_inner())?
        .send()
        .await
        .unwrap()
        .json::<QueryResults>()
        .await.unwrap();

    let json_response = web::Json(response);
    //eventually return a json with all the returned images
    Ok(json_response)
}

async fn app_index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("./frontend/build/index.html")?)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let bind = ("127.0.0.1", 8081);

    HttpServer::new(move || {
        let app = App::new()
            .wrap(Compress::default())
            .wrap(NormalizePath::trim());

        let app = app.wrap(Logger::default());
        let app = app.service(get_images);
        let app = app.default_service(
            Files::new("*", "./frontend/build")
                .index_file("index.html")
                .default_handler(web::get().to(app_index)),
        );
        app
    })
    .workers(1)
    .bind(bind)?
    .run()
    .await
}
