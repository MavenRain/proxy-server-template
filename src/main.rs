extern crate futures;
extern crate hyper;

use hyper::{Client, Request, Server};
use hyper::service::service_fn;
use hyper::rt::{self, Future};
use std::net::SocketAddr;

fn main() {
    let ip = [0, 0, 0, 0];
    let inbound_address = (ip, 8080).into();
    let outbound_address: SocketAddr = (ip, 8081).into();

    let client_main = Client::new();

    let outbound_address_clone = outbound_address.clone();
    let new_service = move || {
        let client = client_main.clone();
        service_fn(move |request| {
            let uri_string = format!("http://{}/{}",
                outbound_address_clone,
                request.uri().path_and_query()
		.map(|x| x.as_str()).unwrap_or(""));
            let uri = uri_string.parse::<String>().unwrap();
            let new_request = Request::builder()
            .method(request.method())
            .uri(uri)
            .body(request.into_body())
            .unwrap();
            client.request(new_request)
        })
    };

    let server = Server::bind(& inbound_address)
        .serve(new_service)
        .map_err(|e| println!("server error: {}", e));

    println!("Listening on http://{}", inbound_address);
    println!("Proxying on http://{}", outbound_address);

    rt::run(server);
}
