#![crate_name = "mesh"]
#![feature(old_io,core)]

// This tells the compiler to look in these additional files for code that's
// part of this module.
mod vector;
mod mesh;
mod stl;
mod amf;

// By default code in a sub-module isn't exposed when someone uses this crate;
// this says to the compiler "I want to use this struct, but all export it to
// people who use me".
pub use self::vector::Vector3D;
pub use self::vector::VertexMap;
pub use self::mesh::Mesh;
pub use self::stl::StlFile;
pub use self::amf::AmfFile;
