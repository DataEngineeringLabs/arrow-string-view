#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringView {
    #[doc = " Length of the string"]
    pub size_: u32,
    #[doc = " The first 4 bytes of the string, whether inline or heap"]
    pub prefix_: [::std::os::raw::c_char; 4usize],
    pub value_: StringView__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union StringView__bindgen_ty_1 {
    #[doc = " For string 12 bytes or less, the remaining bytes beyond the first 4"]
    pub remainder_inlined: [::std::os::raw::c_char; 8usize],
    #[doc = " For strings over 12 bytes in length (points to first byte of string)"]
    pub complete_string_data: *const ::std::os::raw::c_char,
}
