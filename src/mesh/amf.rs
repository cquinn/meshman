#![deny(unused_attributes)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(unused_must_use)]
#![allow(deprecated)]

use std::old_io::{File,IoResult,Reader,Writer};
use std::old_path::Path;
use mesh::Mesh;

pub struct AmfFile;

impl AmfFile {

    pub fn read(_: &mut Reader) -> IoResult<Mesh> {
        panic!("not implemented");
    }

    pub fn write(m: &Mesh, in_file_path: &str) -> IoResult<()> {

        let out_file_name = in_file_path.replace("stl", "amf");
        let path = Path::new(out_file_name);

        let mut out_file = match File::create(&path) {
            Err(why) => return Err(why),
            Ok(x) => x
        };

        let amf_unit = "inch";
        let amf_version = "1.1";
        let amf_oid = "1";
        let amf_mid = "1";
        //let amf_vname = "Test Volume";

        try!(show(&mut out_file, format!("<amf unit='{}' version='{}'>", amf_unit, amf_version)));
        try!(show(&mut out_file, format!("  <object id='{}'>", amf_oid)));
        try!(show(&mut out_file, format!("    <vertices>")));
        for v in m.vertices.iter() {
            try!(show(&mut out_file,
                format!("        <vertex><coordinates><x>{}</x><y>{}</y><z>{}</z></coordinates></vertex>", v.x, v.y, v.z)));
        }
        try!(show(&mut out_file, format!("    </vertices>")));
        try!(show(&mut out_file, format!("    <volume materialid='{}'>", amf_mid)));
        for t in m.facets.iter() {
            try!(show(&mut out_file,
                format!("        <triangle><v1>{}</v1><v2>{}</v2><v3>{}</v3></triangle>", t.v1, t.v2, t.v3)));
        }
        try!(show(&mut out_file, format!("    </volume>")));
        try!(show(&mut out_file, format!("  </object>")));
        try!(show(&mut out_file, format!("</amf>")));
        Ok(())
    }
}

fn show(f: &mut File, s: String) -> IoResult<()> {
    try!(f.write_str(&s));
    try!(f.write_str("\n"));
    Ok(())
}
