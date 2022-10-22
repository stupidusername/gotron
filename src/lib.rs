use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rick_and_morty as rm;
use warp::reject::Reject;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};
use warp::{Filter, Rejection};
use warp_reverse_proxy::{
    extract_request_data_filter, proxy_to_and_forward_response, Body, Headers,
};

pub async fn show_character(id: i64) {
    let c = rm::character::get(id).await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_characters() {
    let c = rm::character::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn show_location(id: i64) {
    let c = rm::location::get(id).await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_locations() {
    let c = rm::location::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn show_episode(id: i64) {
    let c = rm::episode::get(id).await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn list_episodes() {
    let c = rm::episode::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

// TODO: Explain why error type is not encapsuled
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
            eprintln!("Failed to open api-key file: {e}");
            return Err(e);
        }
    };

    if let Err(e) = writeln!(file, "{api_key}") {
        eprintln!("Failed to write api-key file: {e}");
        return Err(e);
    }

    Ok(api_key)
}

#[derive(Debug)]
struct Unauthorized;

impl Reject for Unauthorized {}

pub async fn start_proxy_server() {
    let signup = warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .map(|| {
            let api_key = generate_and_save_api_key().unwrap();
            let mut resp_obj = HashMap::new();
            resp_obj.insert(String::from("api_key"), api_key);
            warp::reply::json(&resp_obj)
        });

    let proxy = warp::path("api")
        .or(warp::path("graphql"))
        .and(auth_validation())
        .and(extract_request_data_filter())
        .and_then(|_, _, path, query, method, mut headers: Headers, body: Body| {
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
        });

    let routes = signup.or(proxy);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn auth_validation() -> impl Filter<Extract = ((),), Error = Rejection> + Copy {
    warp::header::<String>("Authorization").and_then(|api_key: String| async move {
        // This is not good: Errors from validate_api_key() are handled responding 403s.
        if let Ok(true) = validate_api_key(&api_key) {
            Ok(())
        } else {
            Err(warp::reject::custom(Unauthorized))
        }
    })
}

// TODO: Explain why error type is not encapsuled
fn validate_api_key(api_key: &str) -> Result<bool, std::io::Error> {
    let file = OpenOptions::new().create(true).write(true).read(true).open("api-keys.txt")?;
    // TODO: Change or clarify why did I need to use "unwrap()".
    Ok(BufReader::new(file).lines().any(|line| api_key == line.unwrap()))
}