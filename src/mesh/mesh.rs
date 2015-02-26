//#![allow(unstable)]
#![allow(dead_code)]

// This guy depends on a sibling sub-module, so he can use that here.
use vector::Vector3D;
use vector::VertexMap;
use stl::StlFacet;

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

    pub fn new_from_stl(fv: &Vec<StlFacet>, vm: &VertexMap) -> Mesh {
        let vs = vm.vector();
        let fs = Mesh::indexed_vertices(fv, vm);
        Mesh {
            vertices: vs,
            facets: fs,
        }
    }
}

