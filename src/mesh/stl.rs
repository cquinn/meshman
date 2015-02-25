// STL file handling.
//#![allow(unstable)]
#![allow(dead_code)]
#![deny(unused_imports)]

// This guy depends on multiple sibling sub-modules, so he can use that here.
use std::old_io::{IoResult,Reader};
use std::fmt;
use mesh::Mesh;
use vector::Vector3D;
use vector::VertexMap;


pub struct StlFile;

pub struct StlHeader {
    header: [u8; 80],
}

#[derive(PartialEq, Eq, Hash, Copy)]
pub struct StlFacet {
    pub n : Vector3D,
    pub v1: Vector3D,
    pub v2: Vector3D,
    pub v3: Vector3D,
    pub abc: u16,
}

impl fmt::Debug for StlFacet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]({:?}-{:?}-{:?})[{:X}]",
            &self.n, &self.v1, &self.v2, &self.v3, self.abc)
    }
}

impl StlFacet {
    pub fn read(r: &mut Reader) -> IoResult<StlFacet> {
        let n = try!(Vector3D::read(r));
        let v1 = try!(Vector3D::read(r));
        let v2 = try!(Vector3D::read(r));
        let v3 = try!(Vector3D::read(r));
        let abc = try!(r.read_le_u16());
        return Ok(StlFacet { n:n, v1:v1, v2:v2, v3:v3, abc:abc });
    }

    pub fn calculate_normal_vector(&self) -> Vector3D {
        // Dir = (B - A) x (C - A)
        // Norm = Dir / len(Dir)
        let direction = Vector3D::cross( self.v2.minus(self.v1), self.v3.minus(self.v1) );

        // Normalize is optional in our use case
        direction.normalize()
    }
}

impl StlFile {

    pub fn read<R: Reader>(r: &mut R) -> IoResult<Mesh> {

        let mut header = StlHeader { header: [0u8; 80] };
        try!(r.read_at_least(header.header.len(), &mut header.header));

        let hs = String::from_utf8_lossy(&header.header);
        if hs.starts_with("solid ") {
            println!("Is ASCII STL");
        } else {
            println!("Is binary STL");
        }

        StlFile::read_binary(r)
    }

    /*
    fn read_ascii(r: &mut Reader) -> IoResult<Mesh> {
        //solid vcg
        //  facet normal 7.733874e-001 -3.151335e-002 6.331499e-001
        //    outer loop
        //      vertex  2.137833e+001 5.385936e+001 1.350253e+001
        //      vertex  2.341920e+001 4.700068e+001 1.066826e+001
        //      vertex  2.859041e+001 5.985682e+001 4.991545e+000
        //    endloop
        //  endfacet
        //endsolid vcg
        for line in r.lines() {
            print!("{}", line.unwrap());
        }
        Mesh::new() // TODO
    }
    */

    fn read_binary(r: &mut Reader) -> IoResult<Mesh> {
        let facet_count = match r.read_le_u32() {
            Ok(c) => { println!("Facets: {}", c); c},
            Err(e) => { println!("Truncated file: {}", e); 0},
        };

        let mut facets: Vec<StlFacet> = Vec::with_capacity(facet_count as usize);
        let mut vertices = VertexMap::new();
        println!("Collections ready");

        for fi in range(0, facet_count) {
            let f = match StlFacet::read(r) {
                Ok(x) => x,
                Err(e) => panic!("file error: {}", e),
            };
            let v1i = vertices.add(f.v1);
            let v2i = vertices.add(f.v2);
            let v3i = vertices.add(f.v3);
            println!("  Facet[{}]: {:?} => {}-{}-{}", fi, f, v1i, v2i, v3i);
            facets.push(f);
        }
        println!("Vertices: {}", vertices.len());

        Ok(Mesh::new_from_stl(&facets, &vertices))
    }
}
