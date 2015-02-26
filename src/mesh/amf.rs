#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(deprecated)]

use std::old_io::{File,IoResult,Reader};
use std::fmt;
use mesh::Mesh;
use vector::Vector3D;
use vector::VertexMap;

pub struct AmfFile;

impl AmfFile {

    pub fn read(r: &mut Reader) -> IoResult<Mesh> {
        panic!("not implemented");
    }

    pub fn write(m: &Mesh, in_file_path: String) -> () {

        let out_file_name = in_file_path.replace("stl", "amf");
        let path = Path::new(out_file_name);
        let display = path.display();
        
        let mut out_file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.desc),
            Ok(x) => x
        };
        
        let amf_unit = "inch";
        let amf_version = "1.1";
        let amf_oid = "1";
        let amf_mid = "1";
        let amf_vname = "Test Volume";

        AmfFile::show(&mut out_file, format!("<amf unit='{}' version='{}'>", amf_unit, amf_version));
        AmfFile::show(&mut out_file, format!("  <object id='{}'>", amf_oid));
        AmfFile::show(&mut out_file, format!("    <vertices>"));
        for v in m.vertices.iter() {
            AmfFile::show(&mut out_file, format!("        <vertex><coordinates><x>{}</x><y>{}</y><z>{}</z></coordinates></vertex>", v.x, v.y, v.z));
        }
        AmfFile::show(&mut out_file, format!("    </vertices>"));
        AmfFile::show(&mut out_file, format!("    <volume materialid='{}'>", amf_mid));
        for t in m.facets.iter() {
            AmfFile::show(&mut out_file, format!("        <triangle><v1>{}</v1><v2>{}</v2><v3>{}</v3></triangle>", t.v1, t.v2, t.v3));
        }
        AmfFile::show(&mut out_file, format!("    </volume>"));
        AmfFile::show(&mut out_file, format!("  </object>"));
        AmfFile::show(&mut out_file, format!("</amf>"));

    }

    fn show(f: &mut File, s: String) {
        f.write_all(format!("{}\n", s).as_bytes());
    }

}
