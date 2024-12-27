use super::Shape;
use crate::primitives::make_point;
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::ffi;

pub struct Vertex {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Vertex>,
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        let source = ffi::cast_vertex_to_shape(&self.inner);
        let mut copier = ffi::BRepBuilderAPI_Copy_ctor(source, true, false);
        let target = copier.pin_mut().Shape();
        let vertex = ffi::TopoDS_cast_to_vertex(target);
        Vertex::from_vertex(vertex)
    }
}

// You'll see several of these `impl AsRef` blocks for the various primitive
// geometry types. This is for functions which take an Iterator of primitives
// which are either owned or borrowed values. The general pattern looks like this:
//
//     pub fn do_something_with_edges<T: AsRef<Edge>>(edges: impl IntoIterator<Item = T>) {
//         for edge in edges.into_iter() {
//             let edge_ref = edge.as_ref();
//             // Do something with edge_ref
//         }
//     }
impl AsRef<Vertex> for Vertex {
    fn as_ref(&self) -> &Vertex {
        self
    }
}

impl TryFrom<&Shape> for Vertex {
    type Error = cxx::Exception;

    fn try_from(value: &Shape) -> Result<Self, Self::Error> {
        ffi::try_cast_TopoDS_to_vertex(&value.inner).map(Self::from_vertex)
    }
}

impl Vertex {
    pub fn new(point: DVec3) -> Self {
        let mut make_vertex = ffi::BRepBuilderAPI_MakeVertex_gp_Pnt(&make_point(point));
        let vertex = make_vertex.pin_mut().Vertex();
        let inner = ffi::TopoDS_Vertex_to_owned(vertex);

        Self { inner }
    }

    pub(crate) fn from_vertex(vertex: &ffi::TopoDS_Vertex) -> Self {
        let inner = ffi::TopoDS_Vertex_to_owned(vertex);

        Self { inner }
    }

    pub fn x(&self) -> f64 {
        ffi::BRep_Tool_Pnt(&self.inner).X()
    }

    pub fn y(&self) -> f64 {
        ffi::BRep_Tool_Pnt(&self.inner).Y()
    }

    pub fn z(&self) -> f64 {
        ffi::BRep_Tool_Pnt(&self.inner).Z()
    }

    pub fn dist(&self, other: &Vertex) -> f64 {
        let this = ffi::BRep_Tool_Pnt(&self.inner);
        let other = ffi::BRep_Tool_Pnt(&other.inner);
        this.Distance(&other)
    }
}
