//#![allow(unstable)]
#![allow(dead_code)]

use std::fmt;
use vector::Vector3D;

#[derive(PartialEq, Eq, Hash, Copy)]
pub struct Facet {
    pub v1: usize,
    pub v2: usize,
    pub v3: usize,
    pub n: Vector3D
}

impl fmt::Debug for Facet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]({:?}-{:?}-{:?})",
            &self.n, &self.v1, &self.v2, &self.v3)
    }
}

#[derive(PartialEq, Eq, Debug)]
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
