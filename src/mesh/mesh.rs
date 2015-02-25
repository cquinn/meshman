//#![allow(unstable)]
#![allow(dead_code)]

// This guy depends on a sibling sub-module, so he can use that here.
use vector::Vector3D;
use vector::VertexMap;

#[derive(PartialEq, Debug, Eq, Hash, Copy)]
pub struct Facet {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub n: Vector3D
}

pub struct Mesh {
    pub vertices: Vec<Vector3D>,
    pub facets: Vec<Facet>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            facets: Vec::new(),
        }
    }

    pub fn new_from_parts(vs: Vec<Vector3D>, fs: Vec<Facet>) -> Mesh {
        Mesh {
            vertices: vs,
            facets: fs,
        }
    }

}
