#![feature(env)]
#![feature(old_io)]
#![feature(old_path)]

extern crate mesh;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use mesh::POV;

fn main() {

    let mut args = std::env::args();
    args.next();  // skip arg0
    let meshname = args.next().expect(
        "Usage: ./meshman <path/to/mesh>"
    );

    let meshname_copy = meshname.clone();
    
    let meshfile = match File::open(&Path::new(meshname)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    match StlFile::read(&mut BufferedReader::new(meshfile)) {
        Ok(m) => {
            // Add writes to other files here
            POV::write_file(&meshname_copy, m);
        },
        Err(why) => panic!("Can't read mesh: {}", why)
    }
}
