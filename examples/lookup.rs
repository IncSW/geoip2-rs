use geoip2::{City, Reader};
use std::{env, fs, net::IpAddr, str::FromStr};

fn main() {
    let mut args = env::args().skip(1);
    let buffer = fs::read(args.next().unwrap()).unwrap();
    let reader = Reader::<City>::from_bytes(&buffer).unwrap();
    let ip = IpAddr::from_str(&args.next().unwrap()).unwrap();
    let result = reader.lookup(ip).unwrap();
    println!("{:#?}", result);
}
