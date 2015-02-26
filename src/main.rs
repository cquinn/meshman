#![feature(env,old_io,old_path,os)]

extern crate mesh;
extern crate getopts;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use mesh::AmfFile;
use mesh::POV;
use mesh::Mesh;
use mesh::Vector3D;
use getopts::Options;
use std::os;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "input", "File name to process", "FILE");
    opts.optflag("p", "povray", "Export the model into POV-Ray format");
    opts.optflag("a", "amf", "Export the model into AMF format");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args.tail()) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    };

    let export_to_povray = matches.opt_present("p");
    let export_to_amf = matches.opt_present("a");
    
    let input_file = match matches.opt_str("i") {
        Some(x) => x,
        None => panic!("No input file"),
    };
    /*
    let output_file = match matches.opt_str("o") {
        Some(x) => x,
        None => panic!("No output file"),
    };
    */

    let input_file_copy = input_file.clone();
    
    let meshfile = File::open(&Path::new(input_file));

    let file = match StlFile::read(&mut BufferedReader::new(meshfile)) {
        Ok(f) => f,
        Err(e) => { println!("STL file error: {}", e); return; }
    };

    file.println_debug();
    println!("");
    
    let mesh = file.as_mesh();

    // Process free as commands
    let mut commands: Vec<Box<MeshOperation>> = Vec::new();
    let mut iter = matches.free.iter();
    loop {
        let command_name = match iter.next() {
            None => break, // empty
            Some(x) => x.clone(),
        };
        let vector = match iter.next() {
            None => panic!("Every command requires a vector"),
            Some(y) => arg_to_vector(y.clone()),
        };
        let command = match command_name.as_slice() {
            "rotate" => Box::new(RotateOperation { v: vector }),
            _ => panic!("Unknown command: {}", command_name)
        };
        commands.push( command );
    }

    // Engine
    let mut it = commands.iter();
    let mut changed_mesh = mesh;
    loop {
        match it.next() {
            Some(command) => {
                changed_mesh = command.apply(changed_mesh);
            }
            None => { break }
        }
    }

    println!("Mesh: {:?}", &changed_mesh);

    // Do we open the file, if it doesn't exist yet?
//    let generated_file = match File::open(&Path::new(output_file)) {
//        Ok(f) => f,
//        Err(e) => panic!("file error: {}", e),
//    };

    if export_to_povray {
        POV::export_to_pov(&input_file_copy, mesh);
    } else if export_to_amf {
        AmfFile::write(&mesh, input_file_copy);
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

fn arg_to_vector(arg: String) -> Vector3D {
    let parts: Vec<f32> = arg.split(',').filter_map(|s| s.parse::<f32>().ok() ).collect();
    if (parts.len() != 3) {
        panic!("Vector must have three elements: {}", arg)
    };
    Vector3D{x: parts[0], y: parts[1], z: parts[2]}
}

#[test]
fn converts_arg_to_vector() {
    let result = arg_to_vector("1,2,3".to_string());
    assert_eq!(result.x, 1.0);
    assert_eq!(result.y, 2.0);
    assert_eq!(result.z, 3.0);
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [operation vector]", program);
    print!("{}", opts.usage(brief.as_slice()));
}
