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
    let meshfile = File::open(&Path::new(meshname));
    let file = match StlFile::read(&mut BufferedReader::new(meshfile)) {
        Ok(f) => f,
        Err(e) => { println!("STL file error: {}", e); return; }
    };

    file.println_debug();
    println!("");
    
    let mesh = file.as_mesh();
    println!("Mesh: {:?}", &mesh);
        
    POV::write_file(&meshname_copy, mesh);
}
