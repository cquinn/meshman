#![feature(old_io)]
#![feature(old_path)]

use std::fmt;
use std::old_io::File;
use std::old_io::BufferedReader;
use Mesh;
use mesh::Facet;
use vector::Vector3D;
pub struct POV;

impl POV {
    pub fn export_to_pov(file_name: &str, mesh: Mesh) {
        let out_file_name = file_name.replace("stl", "inc");
        let out_file_name_copy = out_file_name.clone();
        let path = Path::new(out_file_name);
        let display = path.display();
        //        let modelname = path.file_name();
        let modelname = "m_model";
        
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.desc),
            Ok(file) => file
        };
        
        file.write_str(&format!("// Source file: {}\n", file_name));
        file.write_str(&format!("# declare {} = mesh {{\n", modelname));
                       
        for facet in mesh.facets.iter() {
            match file.write_str(&POV::facet_to_povstring(&mesh, facet)) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.desc),
                Ok(_) => print!(".")
            }
        }
        
        file.write_str("}\n");

        let template = POV::read_template();
        let first_pass = template.replace("FILE_NAME", &out_file_name_copy);
        let second_pass = first_pass.replace("MODEL_NAME", modelname);

        let modelfilename = file_name.replace("stl", "pov");
        let modelpath = Path::new(modelfilename);
        let mut modelfile = match File::create(&modelpath) {
            Err(why) => panic!("couldn't create {}: {}", modelpath.display(), why),
            Ok(file) => file
        };

        modelfile.write_str(&second_pass);
    }
    
    fn facet_to_povstring(mesh: &Mesh, facet: &Facet) -> String {
        let v1 = mesh.vertices[facet.v1];
        let v2 = mesh.vertices[facet.v2];
        let v3 = mesh.vertices[facet.v3];

        format!("    triangle {{\n        {},\n        {},\n        {}\n    }}\n",
                POV::vertex_to_povstring(v1),
                POV::vertex_to_povstring(v2),
                POV::vertex_to_povstring(v3))
    }

    fn vertex_to_povstring(vector: Vector3D) -> String {
        format!("    <{}, {}, {}>", vector.y, vector.x, vector.z)
    }

    fn read_template() -> String {
        File::open(&Path::new("templates/model.pov")).read_to_string().unwrap()
    }
}

            
