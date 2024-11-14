use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
#[macro_use]
extern crate lazy_static;
use extism::*;
use plugin_common::PluginArgs;
use serde::{Deserialize, Serialize};

// pretend this is redis or something :)
type KVStore = std::collections::BTreeMap<String, Vec<u8>>;
lazy_static! {
    static ref kv_store: UserData<std::collections::BTreeMap<String, Vec<u8>>> =
        UserData::new(KVStore::default());
}

// When a first argument separated with a semicolon is provided to `host_fn` it is used as the
// variable name and type for the `UserData` parameter
host_fn!(kv_read(user_data: KVStore; key: String) -> i64 {
    let kv = user_data.get()?;
    let kv = kv.lock().unwrap();
    let value = kv
        .get(&key)
        .map(|x| i64::from_le_bytes(x.clone().try_into().unwrap()))
        .unwrap_or_else(|| 0i64);
    println!("kv_read: {}", value);
    Ok(value)
});

host_fn!(kv_write(user_data: KVStore; key: String, value: i64) ->Option<i64>{
    let kv = user_data.get()?;
    let mut kv = kv.lock().unwrap();
    let old = kv.insert(key, value.to_le_bytes().to_vec());
    println!("kv_write: {}", value);
    Ok(old)
});

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> String {
    let config = "2024-11-12 18:09:00.000".to_string();
    let date = "ss".to_string();
    let args = PluginArgs::new(date, config);
    let param = serde_json::to_string(&args);
    match param {
        Ok(r) => {
            let url = Wasm::file("target/wasm32-unknown-unknown/debug/hello_plugin.wasm");
            let manifest = Manifest::new([url.with_name("hello")]);

            let mut plugin = PluginBuilder::new(manifest)
                .with_wasi(true)
                .with_function(
                    "kv_read",
                    [ValType::I64],
                    [ValType::I64],
                    kv_store.clone(),
                    kv_read,
                )
                .with_function(
                    "kv_write",
                    [ValType::I64, ValType::I64],
                    [ValType::I64],
                    kv_store.clone(),
                    kv_write,
                )
                .build()
                .unwrap();

            let res = plugin.call::<&str, &str>("process", &r).unwrap();
            res.to_owned()
        }
        Err(e) => {
            // TODO
            "".to_string()
        }
    }
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
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
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
