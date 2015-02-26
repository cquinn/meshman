// STL file handling.
//#![allow(unstable)]
#![allow(dead_code)]
#![deny(unused_imports)]

// This guy depends on multiple sibling sub-modules, so he can use that here.
use std::old_io::{IoResult,Reader};
use std::fmt;
use mesh::Mesh;
use mesh::Facet;
use vector::Vector3D;
use vector::VertexMap;

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

pub struct StlFile {
    pub header: [u8; 80],
    pub facets: Vec<StlFacet>,
    pub vertices: VertexMap,
}

impl StlFile {

    pub fn read<R: Reader>(r: &mut R) -> IoResult<StlFile> {
        let mut file = StlFile {
            header: [0u8; 80],
            facets: Vec::new(),
            vertices: VertexMap::new()
        };
        try!(r.read_at_least(file.header.len(), &mut file.header));
        // TODO: select which form to read: binary or ascii
        try!(file.read_binary(r));
        Ok(file)
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

    fn read_binary(&mut self, r: &mut Reader) -> IoResult<&StlFile> {
        let facet_count = try!(r.read_le_u32());
        for _ in range(0, facet_count) {
            let f = try!(StlFacet::read(r));
            self.vertices.add(f.v1);
            self.vertices.add(f.v2);
            self.vertices.add(f.v3);
            self.facets.push(f);
        }
        Ok(self)
    }

    pub fn kind(&self) -> String {
        let hs = String::from_utf8_lossy(&self.header);
        if hs.starts_with("solid ") {
            "ASCII".to_string()
        } else {
            "binary".to_string()
        }
    }

    pub fn as_mesh(&self) -> Mesh {
        new_mesh(&self.facets, &self.vertices)
    }

    pub fn println_debug(&self) {
        println!("Is {} STL", self.kind());
        println!("Facets: {}", self.facets.len());
        for f in self.facets.iter() {
            let v1i = self.vertices.get(&f.v1);
            let v2i = self.vertices.get(&f.v2);
            let v3i = self.vertices.get(&f.v3);
            println!("  Facet: {:?} => {}-{}-{}", f, v1i, v2i, v3i);
        }
        println!("Vertices: {}", self.vertices.len());
    }
}

fn new_mesh(fv: &Vec<StlFacet>, vm: &VertexMap) -> Mesh {
    Mesh::new_from_parts(vm.vector(), indexed_vertices(fv, vm))
}

fn indexed_vertices(fv: &Vec<StlFacet>, vm: &VertexMap) -> Vec<Facet> {
    let mut v: Vec<Facet> = Vec::with_capacity(fv.len());
    for f in fv.iter() {
        let v1 = vm.get(&f.v1);
        let v2 = vm.get(&f.v2);
        let v3 = vm.get(&f.v3);
        // I wish there was a better place to put this
        let n = f.calculate_normal_vector();
        v.push(Facet{
            v1: v1,
            v2: v2,
            v3: v3,
            n: n,
        })
    }
    v
}
