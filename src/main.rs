use clap::{App, Arg};
use smoltcp::phy::TapInterface;
use url::Url;

mod dns;
mod ethernet;
mod http;

fn main(){
    let app = App::new("mget")
        .about("Get a webpage, manually")
        .arg(Arg::with_name("url").required(true))
        .arg(Arg::with_name("tap-device").required(true))
        .arg(Arg::with_name("dns-server").default_value("1.1.1.1"))
        .get_matches();

    let url_text =  app.value_of("url").unwrap();
    let dns_server_text = app.value_of("dns-server").unwrap();
    let tap_text = app.value_of("tap-device").unwrap();

    let url = Url::parse(url_text).expect("error: unable to parse <url> as a URL");
    if url.scheme() != "http" {
        eprintln!("Error: only HTTP protocol supported");
        return;
    }

    let tap = TapInterface::new(&tap_text).expect("Error: unable to use <tap-device> as network interface");

    let domain_name = url.host_str().expect("Error: domain name expected");

    let dns_server: std::net::Ipv4Addr = dns_server_text.parse().expect("Error: unnable to parse <dns-server> as an IPv4 address");

    println!("URL: {}, Domain: {}, TAP: {:?}, DNS: {}", url, domain_name, tap, dns_server);
}