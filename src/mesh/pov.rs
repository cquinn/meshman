#![feature(old_io)]
#![feature(old_path)]

use std::fmt;
use std::old_io::File;
use Mesh;
use mesh::Facet;
use vector::Vector3D;
pub struct POV;

impl POV {
    pub fn write_file(file_name: &str, mesh: Mesh) {
        let out_file_name = file_name.replace("stl", "inc");
        let path = Path::new(out_file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.desc),
            Ok(file) => file
        };

        file.write_str(&format!("// Source file: {}\n", file_name));
        file.write_str("# declare m_mesh = mesh {\n");

        for facet in mesh.facets.iter() {
            match file.write_str(&POV::facet_to_povstring(&mesh, facet)) {
                Err(why) => panic!("couldn't write to {}: {}", display, why.desc),
                Ok(_) => print!(".")
            }
        }

        file.write_str("}\n");
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
}

            
