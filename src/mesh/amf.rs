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

    pub fn write(m: Mesh, f: &mut File) -> () {

        let amf_unit = "inch";
        let amf_version = "1.1";
        let amf_oid = "1";
        let amf_mid = "1";
        let amf_vname = "Test Volume";

        AmfFile::show(f, format!("<amf unit='{}' version='{}'>", amf_unit, amf_version));
        AmfFile::show(f, format!("  <object id='{}'>", amf_oid));
        AmfFile::show(f, format!("    <vertices>"));
        for v in m.vertices.iter() {
            AmfFile::show(f, format!("        <vertex><coordinates><x>{}</x><y>{}</y><z>{}</z></coordinates></vertex>", v.x, v.y, v.z));
        }
        AmfFile::show(f, format!("    </vertices>"));
        AmfFile::show(f, format!("    <volume materialid='{}'>", amf_mid));
        for t in m.facets.iter() {
            AmfFile::show(f, format!("        <triangle><v1>{}</v1><v2>{}</v2><v3>{}</v3></triangle>", t.v1, t.v2, t.v3));
        }
        AmfFile::show(f, format!("    </volume>"));
        AmfFile::show(f, format!("  </object>"));
        AmfFile::show(f, format!("</amf>"));

    }

    fn show(f: &mut File, s: String) {
        f.write_all(format!("{}\n", s).as_bytes());
    }

}
