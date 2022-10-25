use cxx::UniquePtr;
use opencascade_sys::ffi::{
    new_point, new_shape, write_stl, BRepFilletAPI_MakeFillet_ctor, BRepMesh_IncrementalMesh_ctor,
    BRepPrimAPI_MakeBox_ctor, StlAPI_Writer_ctor, TopAbs_ShapeEnum, TopExp_Explorer_ctor,
    TopoDS_Shape, TopoDS_cast_to_edge,
};
use std::path::Path;

pub struct Shape {
    shape: UniquePtr<TopoDS_Shape>,
}

impl Shape {
    pub fn make_box() -> Self {
        let point = new_point(0.0, 0.0, 0.0);
        let mut my_box = BRepPrimAPI_MakeBox_ctor(&point, 1.0, 1.0, 1.0);
        let shape = new_shape(my_box.pin_mut().Shape());

        Self { shape }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) {
        let mut stl_writer = StlAPI_Writer_ctor();
        let triangulation = BRepMesh_IncrementalMesh_ctor(&self.shape, 0.001);
        let success = write_stl(
            stl_writer.pin_mut(),
            triangulation.Shape(),
            path.as_ref().to_string_lossy().to_string(),
        );

        println!("Done! Success = {}", success);
    }

    pub fn fillet_edges(&mut self, radius: f64) {
        let mut make_fillet = BRepFilletAPI_MakeFillet_ctor(&self.shape);
        let mut edge_explorer = TopExp_Explorer_ctor(&self.shape, TopAbs_ShapeEnum::TopAbs_EDGE);

        while edge_explorer.More() {
            let edge = TopoDS_cast_to_edge(edge_explorer.Current());
            make_fillet.pin_mut().add_edge(radius, edge);
            edge_explorer.pin_mut().Next();
        }

        let filleted_shape = make_fillet.pin_mut().Shape();

        self.shape = new_shape(filleted_shape);
    }
}
