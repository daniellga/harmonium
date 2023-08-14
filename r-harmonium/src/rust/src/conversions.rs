use extendr_api::{prelude::*, AsTypedSlice};

pub(crate) trait RobjConversions<'a, A>
where
    A: PartialEq + Copy + 'a,
    Self: AsTypedSlice<'a, A>,
{
    fn robj_to_scalar(&'a self) -> A;
    fn robj_to_slice(&'a self) -> &[A];
}

impl<'a, A> RobjConversions<'a, A> for Robj
where
    A: PartialEq + Copy + 'a,
    Robj: AsTypedSlice<'a, A>,
{
    fn robj_to_scalar(&'a self) -> A {
        assert!(self.len() == 1);
        let slice: &'a [A] = self.robj_to_slice();
        slice[0]
    }

    fn robj_to_slice(&'a self) -> &[A] {
        let slice: &'a [A] = self.as_typed_slice().unwrap();
        slice
    }
}
