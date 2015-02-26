#![feature(env,old_io,old_path,os)]

extern crate mesh;
extern crate getopts;
extern crate nalgebra;

use std::old_io::BufferedReader;
use std::old_io::fs::File;
use mesh::StlFile;
use mesh::AmfFile;
use mesh::POV;
use mesh::Mesh;
use mesh::Vector3D;
use getopts::Options;
use std::os;
use nalgebra::*;

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "input", "File name to process", "FILE");
    opts.optflag("p", "povray", "Export the model into POV-Ray format");
    opts.optflag("a", "amf", "Export the model into AMF format");
    opts.optflag("v", "view", "Print the model to the console");
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
    let export_to_console = matches.opt_present("v");

    let input_file = match matches.opt_str("i") {
        Some(x) => x,
        None => { println!("No input file"); return; },
    };

    let input_file_copy = input_file.clone();

    let meshfile = File::open(&Path::new(input_file));

    let file = match StlFile::read(&mut BufferedReader::new(meshfile)) {
        Ok(f) => f,
        Err(e) => { println!("STL read error: {}", e); return; }
    };

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
            None => { println!("Every command requires a vector"); return; },
            Some(y) => arg_to_vector(y.clone()),
        };
        let command:Box<MeshOperation> = match command_name.as_slice() {
            "rotate" => Box::new(RotateOperation { v: vector }),
            "scale" => Box::new(ScaleOperation { v: vector }),
            "translate" => Box::new(TranslateOperation { v: vector }),
            _ => { println!("Unknown command: {}", command_name); return; }
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

    if export_to_povray {
        POV::export_to_pov(&input_file_copy, &changed_mesh);
    } else if export_to_amf {
        AmfFile::write(&changed_mesh, input_file_copy);
    } else if export_to_console {
        //file.println_debug();
        println!("Mesh: {:?}", &changed_mesh);
    };

}

// Command pattern
trait MeshOperation {
    fn apply(&self, mesh: Mesh) -> Mesh;
}

pub struct RotateOperation {
    v: Vector3D,
}
pub struct ScaleOperation {
    v: Vector3D,
}
pub struct TranslateOperation {
    v: Vector3D,
}

impl MeshOperation for RotateOperation {
    fn apply(&self, mesh: Mesh) -> Mesh {
        let rot = Vec3::new(self.v.x, self.v.y, self.v.z);

        let v3s = mesh.vertices.iter()
            .map(|v| Vec3::new(v.x, v.y, v.z) )
            .map(|v3| rot.rotate(&v3))
            .map(|v3| Vector3D {x: v3.x, y: v3.y, z: v3.z} )
            .collect();

        //println!("{:?}", v3s);
        return Mesh::new_from_parts(v3s, mesh.facets);
    }
}
impl MeshOperation for ScaleOperation {
    fn apply(&self, mesh: Mesh) -> Mesh {
        let rot = Vec3::new(self.v.x, self.v.y, self.v.z);

        let v3s = mesh.vertices.iter()
        .map(|v| Vec3::new(v.x, v.y, v.z) )
        .map(|v3| rot.rotate(&v3))
        .map(|v3| Vector3D {x: v3.x, y: v3.y, z: v3.z} )
        .collect();

        //println!("{:?}", v3s);
        return Mesh::new_from_parts(v3s, mesh.facets);
    }
}

impl MeshOperation for TranslateOperation {
    fn apply(&self, mesh: Mesh) -> Mesh {
        let rot = Vec3::new(self.v.x, self.v.y, self.v.z);

        let v3s = mesh.vertices.iter()
        .map(|v| Vec3::new(v.x, v.y, v.z) )
        .map(|v3| rot.rotate(&v3))
        .map(|v3| Vector3D {x: v3.x, y: v3.y, z: v3.z} )
        .collect();

        //println!("{:?}", v3s);
        return Mesh::new_from_parts(v3s, mesh.facets);
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

#[test]
fn able_to_rotate() {
    let a = Vec3::new(1.0, 1.0, 1.0);
    let c = Vec3::new(1.0, 1.0, 1.0);
    let d = a.rotate(&c);

    assert_eq!(a.as_array(), d.as_array());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [operation vector]", program);
    print!("{}", opts.usage(brief.as_slice()));
}
