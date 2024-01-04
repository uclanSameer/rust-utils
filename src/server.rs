use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub async fn start_server() {
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    println!("payload: {:?}", &payload);
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u64,
    username: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aa() {
        let send = reqwest::Client::new()
            .post("http://localhost:3000/users")
            .json(&CreateUser {
                username: "Sameer".to_string(),
            })
            .send();

        // wait for the response
        let res = tokio_test::block_on(send).unwrap();

        let json = res.json::<User>();

        // print the response
        let actual_user = tokio_test::block_on(json).unwrap();
        println!("actual_user: {:?}", actual_user);
    }
}
