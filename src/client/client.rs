use std::borrow::Borrow;
use std::io::Read;

use hyper::Client;
use hyper::header::Connection;

use client::urlbuilder;

pub fn get_by_http(hostname: &str, resource: &str) -> String {
    get(urlbuilder::get_http_url(hostname, resource).borrow())
}

pub fn get_by_https(hostname: &str, resource: &str) -> String {
    get(urlbuilder::get_https_url(hostname, resource).borrow())
}


fn get(url: &str) -> String {
    let client = Client::new();

    let mut res = client.get(url)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    body
}

