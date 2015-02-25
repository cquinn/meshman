#![feature(old_io)]
extern crate mesh;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::Mesh;

fn main() {
    let args = std::os::args();
    let meshname = args.as_slice().get(1).expect(
        "Usage: ./meshman <path/to/mesh>"
    ).as_slice();
    let meshfile = match File::open(&Path::new(meshname)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let mesh = Mesh::read(&mut BufferedReader::new(meshfile));
}
