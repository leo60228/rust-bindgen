/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


/**
 * <div rustbindgen replaces="nsTArray"></div>
 */
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsTArray<T> {
    pub y: ::std::os::raw::c_uint,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct Test {
    pub a: nsTArray<::std::os::raw::c_long>,
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(::std::mem::size_of::<Test>() , 4usize);
    assert_eq!(::std::mem::align_of::<Test>() , 4usize);
}
impl Clone for Test {
    fn clone(&self) -> Self { *self }
}