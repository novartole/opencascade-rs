#include <wrapper.hxx>

// Point stuff
std::unique_ptr<gp_Pnt> new_point(double x, double y, double z) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(x, y, z));
}

// Segment Stuff
std::unique_ptr<Geom_TrimmedCurve> new_segment(const gp_Pnt& p1, const gp_Pnt& p2) {
  // TODO(bschwind) - This new is probably leaking memory.
  auto new_segment_result = new GC_MakeSegment(p1, p2);
  auto segment = new_segment_result->Value();
  return std::unique_ptr<Geom_TrimmedCurve>(segment.get());
}

// Arc stuff
std::unique_ptr<Geom_TrimmedCurve> new_arc_of_circle(const gp_Pnt& p1, const gp_Pnt& p2, const gp_Pnt& p3) {
  auto new_arc_result = new GC_MakeArcOfCircle(p1, p2, p3);
  auto new_arc = new_arc_result->Value();
  return std::unique_ptr<Geom_TrimmedCurve>(new_arc.get());
}

// Topo stuff
std::unique_ptr<TopoDS_Edge> make_edge(const Geom_TrimmedCurve &geom_curve) {
  // TODO - Is this curve_handle is destroying the geom_curve?
  Handle(Geom_TrimmedCurve) curve_handle = opencascade::handle<Geom_TrimmedCurve>(&geom_curve);
  auto make_edge_result = BRepBuilderAPI_MakeEdge(curve_handle);
  auto edge = new TopoDS_Edge(make_edge_result.Edge());

  return std::unique_ptr<TopoDS_Edge>(edge);
}
