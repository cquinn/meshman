// Vector sub-module.
//#![allow(unstable)]
#![allow(dead_code)]
#![deny(unused_imports)]

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::mem::{transmute};
use std::num::Float;
use std::old_io::{IoResult,Reader};

#[derive(PartialEq, PartialOrd, Copy)] //Show,
pub struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

// PartialEq is derived above, but we need to impl Eq since floats don't get
// it by default. This function is just a
impl Eq for Vector3D {
    #[inline(always)]
    fn assert_receiver_is_total_eq(&self) {}
}

impl fmt::Debug for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Vector3D {
    pub fn read(r: &mut Reader) -> IoResult<Vector3D> {
        let xr = try!(r.read_le_f32());
        let yr = try!(r.read_le_f32());
        let zr = try!(r.read_le_f32());
        return Ok(Vector3D { x: xr, y: yr, z: zr });
    }

    pub fn minus(&self, o: Vector3D) -> Vector3D {
        let x = self.x - o.x;
        let y = self.y - o.y;
        let z = self.z - o.z;
        Vector3D {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn cross(a: Vector3D, b:Vector3D) -> Vector3D {
        let cx = a.y * b.z - a.z * b.y;
        let cy = a.z * b.x - a.x * b.z;
        let cz = a.x * b.y - a.y * b.x;
        Vector3D {
        x: cx,
        y: cy,
        z: cz
        }
    }

    pub fn normalize(&self) -> Vector3D {
        // length = sqrt((ax * ax) + (ay * ay) + (az * az))
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let x = self.x / length;
        let y = self.y / length;
        let z = self.z / length;
        Vector3D {
            x: x,
            y: y,
            z: z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::Float;


    #[test]
    fn vectors_can_be_subtracted() {
        let l = Vector3D {x:1.0, y:1.0, z:1.0};
        let r = Vector3D {x:2.0, y:3.0, z:4.0};
        let result = l.minus(r);
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn vectors_can_cross_product() {
        let l = Vector3D {x:3.0, y:-3.0, z:1.0};
        let r = Vector3D {x:4.0, y:9.0, z:2.0};

        let result = Vector3D::cross(l, r);
        assert_eq!(result.x, -15.0);
        assert_eq!(result.y, -2.0);
        assert_eq!(result.z, 39.0);
    }

    #[test]
    fn vectors_can_normalize() {
        let r = Vector3D {x:3.0, y:1.0, z:2.0};

        let result = r.normalize();
        //assert_eq!(format!("", result.x), 0.802);
        //assert_eq!(result.y.round(), 0.267);
        //assert_eq!(result.y.round(), 0.534);

    }
}

// Implement Hash since there is no default for f32. We'll just hash the bits
// since we know the f32s will all be canonical from reading.
impl Hash for Vector3D {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        //println!("V3D hash {:p}", self);
        let x: u32 = unsafe { transmute(self.x) };
        x.hash(state);
        let y: u32 = unsafe { transmute(self.y) };
        y.hash(state);
        let z: u32 = unsafe { transmute(self.z) };
        z.hash(state);
        //println!("V3D hash done {:p}", self);
    }
}

pub struct VertexMap {
    vertices: HashMap<Vector3D,usize>,
}

impl VertexMap {
    pub fn new() -> VertexMap {
        VertexMap {
            vertices: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }

    pub fn get(&self, v3d: &Vector3D) -> usize {
        *self.vertices.get(v3d).unwrap()
    }

    pub fn add(&mut self, vertex: Vector3D) -> usize {
        if self.vertices.contains_key(&vertex) {
            //println!("add existing {}", vector3d_str(&vertex));
            *self.vertices.get(&vertex).unwrap()
        } else {
            //println!("add new {}", vector3d_str(&vertex));
            let idx = self.vertices.len();
            self.vertices.insert(vertex, idx);
            idx
        }
    }

    pub fn vector(&self) -> Vec<Vector3D> {
        let mut v: Vec<Vector3D> = Vec::with_capacity(self.vertices.len());
        for (v3d, _) in self.vertices.iter() {
            v.push(*v3d);  // Funkiness just to get v expanded. Below writes the real values.
        }
        for (v3d, idx) in self.vertices.iter() {
            //println!("v3d:{:?} idx:{}", v3d, idx);
            v[*idx] = *v3d;
        }
        //for i in 0..v.len() {
        //    println!("v3d[{}]: {:?}", i, v[i]);
        //}
        v
    }
}
