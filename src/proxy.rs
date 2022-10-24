use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::net::SocketAddr;
use warp::reject;
use warp::{http::StatusCode, reject::Reject, Filter, Rejection, Reply};
use warp_reverse_proxy::{
    extract_request_data_filter, proxy_to_and_forward_response, Body, Headers,
};

const API_KEYS_FILE: &str = "api-keys.txt";
const SERVER_SOCKET_ADDR: &str = "127.0.0.1:8080";

#[derive(Debug)]
struct InternalServerError;

impl Reject for InternalServerError {}

#[derive(Debug)]
struct Unauthorized;

impl Reject for Unauthorized {}

fn generate_and_save_api_key() -> Result<String, std::io::Error> {
    let api_key = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(API_KEYS_FILE)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open api-keys file: {e}");
            return Err(e);
        }
    };

    if let Err(e) = writeln!(file, "{api_key}") {
        eprintln!("Failed to write api-keys file: {e}");
        return Err(e);
    }

    Ok(api_key)
}

fn auth_validation() -> impl Filter<Extract = ((),), Error = Rejection> + Copy {
    warp::header::<String>("Authorization").and_then(|api_key: String| async move {
        match validate_api_key(&api_key) {
            Ok(true) => Ok(()),
            Ok(false) => Err(warp::reject::custom(Unauthorized)),
            Err(_) => Err(warp::reject::custom(InternalServerError)),
        }
    })
}

fn validate_api_key(api_key: &str) -> Result<bool, std::io::Error> {
    let file = match OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(API_KEYS_FILE)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open api-keys file: {e}");
            return Err(e);
        }
    };

    Ok(BufReader::new(file).lines().any(|line| match line {
        Ok(content) => api_key == content,
        Err(e) => {
            eprintln!("Failed to read api-keys file: {e}");
            false
        }
    }))
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status("Not Found", StatusCode::NOT_FOUND))
    } else if let Some(_) = err.find::<Unauthorized>() {
        Ok(warp::reply::with_status(
            "Unauthorized",
            StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        Ok(warp::reply::with_status(
            "Method Not Allowed",
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else if let Some(_) = err.find::<reject::MissingHeader>() {
        Ok(warp::reply::with_status(
            "Missing Headers",
            StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

pub async fn start_proxy_server() {
    let signup = warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .and_then(|| async {
            match generate_and_save_api_key() {
                Ok(api_key) => {
                    let mut resp_obj = HashMap::new();
                    resp_obj.insert(String::from("api_key"), api_key);
                    Ok(warp::reply::json(&resp_obj))
                }
                Err(_) => Err(warp::reject::custom(InternalServerError)),
            }
        });

    let proxy = warp::path("api")
        .or(warp::path("graphql"))
        .and(auth_validation())
        .and(extract_request_data_filter())
        .and_then(
            |_, _, path, query, method, mut headers: Headers, body: Body| {
                // The Rick and Morty API denies the request if this header is forwarded.
                headers.remove("Host");
                // The authorization header should not be sent to the Rick and Morty API.
                headers.remove("Authorization");
                proxy_to_and_forward_response(
                    "https://rickandmortyapi.com/".to_string(),
                    "proxy/".to_string(),
                    path,
                    query,
                    method,
                    headers,
                    body,
                )
            },
        );

    let routes = signup.or(proxy).recover(handle_rejection);

    println!("Starting proxy server on {SERVER_SOCKET_ADDR}");
    warp::serve(routes)
        .run(SERVER_SOCKET_ADDR.parse::<SocketAddr>().unwrap())
        .await;
}
