use cloudflare_zlib::inflate;
use std::io::{stdin, Read};
use std::str;

fn main() {
    let mut buf = Vec::new();
    stdin().read_to_end(&mut buf).expect("Could not read input");
    let inflated_data = inflate(buf.as_slice());
    let res = &inflated_data.unwrap()[..];
    match str::from_utf8(res) {
        Ok(contents) => println!("Output: {:?}", contents),
        Err(e) => println!("Error: {:?}", e),
    };
}
