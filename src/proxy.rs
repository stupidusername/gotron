use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use warp::Reply;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use warp::reject::Reject;
use warp::{Filter, Rejection, http::StatusCode};
use warp_reverse_proxy::{
    extract_request_data_filter, proxy_to_and_forward_response, Body, Headers,
};

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
        .open("api-keys.txt")
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
        .open("api-keys.txt")
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
    if let Some(_) = err.find::<Unauthorized>() {
        Ok(warp::reply::with_status("Unauthorized", StatusCode::UNAUTHORIZED))
    } else {
        Ok(warp::reply::with_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR))
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

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
