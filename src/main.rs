#![feature(old_io)]
extern crate mesh;
extern crate getopts;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use mesh::Mesh;
use mesh::vector::Vector3D;
use getopts::Options;
use std::os;
use std::f32;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("i", "input", "File name to process", "FILE");
    opts.reqopt("o", "output", "File name to output", "FILE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    };

    let input_file = match matches.opt_str("i") {
        Some(x) => x,
        None => panic!("No input file"),
    };
    let meshfile = match File::open(&Path::new(input_file)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
    let mesh = StlFile::read(&mut BufferedReader::new(meshfile));

    // Process free as commands
    let mut commands: Vec<&MeshOperation> = Vec::new();
    loop {
        let command_name = match matches.free.pop() {
            None => break, // empty
            Some(x) => x,
        };
        let vector = match matches.free.pop() {
            None => panic!("Every command requires a vector"),
            Some(x) => {
                let parts: Vec<f32> = x.split(',').filter_map(|s| s.parse::<f32>().ok() ).collect();
                if (parts.len() != 3) {
                    panic!("Vector must have three elements")
                };
                Vector3D{x: parts[0], y: parts[1], z: parts[2]}
            },
        };
        let command = match command_name.as_slice() {
            "rotate" => RotateOperation { v: vector }
        };
        commands.push(&command);
    }

    // Do we open the file, if it doesn't exist yet?
    let output_file = match matches.opt_str("o") {
        Some(x) => x,
        None => panic!("No output file"),
    };
    let generated_file = match File::open(&Path::new(output_file)) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };
}

// Command pattern
trait MeshOperation {
    fn apply(&self, mesh: Mesh) -> Mesh;
}

pub struct RotateOperation {
    v: Vector3D,
}

impl MeshOperation for RotateOperation {
    fn apply(&self, mesh: Mesh) -> Mesh {
        return Mesh::new();
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [operation vector]", program);
    print!("{}", opts.usage(brief.as_slice()));
}