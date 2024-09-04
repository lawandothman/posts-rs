#[macro_use]
extern crate rocket;

use reqwest::Client;
use rocket::{
    http::Status,
    response::status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(rename = "userId")]
    user_id: usize,
    id: usize,
    title: String,
    body: String,
}

#[get("/posts/<id>")]
async fn get_post(client: &State<Client>, id: usize) -> Result<Json<Post>, status::Custom<String>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);

    client
        .get(&url)
        .send()
        .await
        .map_err(|e| status::Custom(Status::ServiceUnavailable, e.to_string()))?
        .error_for_status()
        .map_err(|e| status::Custom(Status::BadRequest, e.to_string()))?
        .json::<Post>()
        .await
        .map(Json)
        .map_err(|e| status::Custom(Status::InternalServerError, e.to_string()))
}

#[launch]
fn rocket() -> _ {
    let client = Client::new();

    rocket::build()
        .manage(client)
        .mount("/api", routes![get_post])
}
