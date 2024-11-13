use super::{Face, IntoShape, Shape, Solid};
use crate::primitives::Wire;
use cxx::UniquePtr;
use opencascade_sys::ffi;

pub struct Shell {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Shell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl TryFrom<&Shape> for Shell {
    type Error = cxx::Exception;

    fn try_from(value: &Shape) -> Result<Self, Self::Error> {
        ffi::try_cast_TopoDS_to_shell(&value.inner).map(Self::from_shell)
    }
}

impl Shell {
    pub(crate) fn from_shell(shell: &ffi::TopoDS_Shell) -> Self {
        let inner = ffi::TopoDS_Shell_to_owned(shell);

        Self { inner }
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = false;
        let mut make_loft = ffi::BRepOffsetAPI_ThruSections_ctor(is_solid);

        for wire in wires.into_iter() {
            make_loft.pin_mut().AddWire(&wire.as_ref().inner);
        }

        // Set CheckCompatibility to `true` to avoid twisted results.
        make_loft.pin_mut().CheckCompatibility(true);

        let shape = make_loft.pin_mut().Shape();
        let shell = ffi::TopoDS_cast_to_shell(shape);

        Self::from_shell(shell)
    }

    pub fn volume(&self, face: &Face) -> Solid {
        // create volume maker
        let mut maker = ffi::BOPAlgo_MakerVolume_ctor();

        // set arguments to make solid from
        let mut arguments = ffi::new_list_of_shape();
        for shape in [self.into_shape(), face.into_shape()] {
            ffi::shape_list_append_shape(arguments.pin_mut(), &shape.as_ref().inner);
        }
        maker.pin_mut().SetArguments(&arguments);

        // perform the opearation
        maker.pin_mut().Perform(&ffi::Message_ProgressRange_ctor());
        // cast result to solid according to doc
        let genaral_shape = ffi::BOPAlgo_MakerVolume_Shape(&maker);
        let solid = ffi::TopoDS_cast_to_solid(genaral_shape);
        Solid::from_solid(solid)
    }
}
