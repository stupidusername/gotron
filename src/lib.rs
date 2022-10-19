use rick_and_morty as rm;
use warp::{hyper::body::Bytes, Filter, Rejection, Reply, http::Response};
use warp_reverse_proxy::reverse_proxy_filter;

async fn log_response(response: Response<Bytes>) -> Result<impl Reply, Rejection> {
    println!("{:?}", response);
    Ok(response)
}

pub async fn list_characters() {
    let c = rm::character::get_all().await;
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

pub async fn list_episodes() {
    let c = rm::episode::get_all().await;
    match c {
        Ok(res) => println!("{:?}", res),
        Err(e) => println!("{:?}", e),
    }
}

pub async fn start_proxy_server() {
    let hello_world = warp::path::end().map(|| "Hello, World at root!");

    let hi = warp::path("hi").map(|| "Hello, World!");
    let proxy = warp::path!("proxy" / ..).and(reverse_proxy_filter(
        "proxy/".to_string(),
        "https://rickandmortyapi.com/".to_string(),
    )).and_then(log_response);

    let routes = warp::get().and(hello_world.or(hi).or(proxy));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
