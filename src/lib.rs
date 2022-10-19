use rick_and_morty as rm;
use warp::Filter;
use warp_reverse_proxy::{
    extract_request_data_filter, proxy_to_and_forward_response, Body, Headers,
};

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
    let proxy = warp::path!("proxy" / ..)
        .and(extract_request_data_filter())
        .and_then(|path, query, method, mut headers: Headers, body: Body| {
            // The rick and morty API denies the request if this header is forwarded.
            headers.remove("Host");
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

    let routes = warp::get().and(hello_world.or(hi).or(proxy));

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
