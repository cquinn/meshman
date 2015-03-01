
use std::old_io::{File,IoResult};
use Mesh;
use mesh::Facet;
use vector::Vector3D;

pub struct POV;

impl POV {
    pub fn write(mesh: &Mesh, file_name: &str) -> IoResult<()> {
        let out_file_name = file_name.replace("stl", "inc");
        let path = Path::new(out_file_name.clone());

        let mut file = match File::create(&path) {
            Err(why) => { return Err(why); },
            Ok(file) => file
        };

        //let modelname = path.file_name();
        let modelname = "m_model";

        try!(file.write_str(&format!("// Source file: {}\n", file_name)));
        try!(file.write_str(&format!("# declare {} = mesh {{\n", modelname)));

        for facet in mesh.facets.iter() {
            match file.write_str(&POV::facet_to_povstring(&mesh, facet)) {
                Err(why) => { return Err(why); },
                Ok(_) => print!(".")
            }
        }
        println!("done.");

        try!(file.write_str("}\n"));

        let template = POV::read_template();
        let first_pass = template.replace("FILE_NAME", &out_file_name);
        let second_pass = first_pass.replace("MODEL_NAME", modelname);

        let modelfilename = file_name.replace("stl", "pov");
        let modelpath = Path::new(modelfilename);
        let mut modelfile = match File::create(&modelpath) {
            Err(why) => { return Err(why); },
            Ok(file) => file
        };

        try!(modelfile.write_str(&second_pass));
        Ok(())
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
