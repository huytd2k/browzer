mod net;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let url = &args[1]; 
    let content = net::send_http_request(&url).unwrap();

    println!("{}", content);
}
