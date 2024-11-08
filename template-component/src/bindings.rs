#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod cd40 {
        #[allow(dead_code)]
        pub mod helm {
            #[allow(dead_code, clippy::all)]
            pub mod template {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_execute_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::execute(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec4 = result1;
                    let len4 = vec4.len();
                    let layout4 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec4.len() * 8,
                        4,
                    );
                    let result4 = if layout4.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout4);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec4.into_iter().enumerate() {
                        let base = result4.add(i * 8);
                        {
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *base.add(4).cast::<usize>() = len3;
                            *base.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    }
                    *ptr2.add(4).cast::<usize>() = len4;
                    *ptr2.add(0).cast::<*mut u8>() = result4;
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_execute<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base4 = l0;
                    let len4 = l1;
                    for i in 0..len4 {
                        let base = base4.add(i * 8);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                        }
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
                }
                pub trait Guest {
                    fn execute(chart_zip: _rt::Vec<u8>) -> _rt::Vec<_rt::String>;
                }
                #[doc(hidden)]
                macro_rules! __export_cd40_helm_template_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "cd40:helm/template#execute"]
                        unsafe extern "C" fn export_execute(arg0 : * mut u8, arg1 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_execute_cabi::<$ty > (arg0, arg1) } #[export_name =
                        "cabi_post_cd40:helm/template#execute"] unsafe extern "C" fn
                        _post_return_execute(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_execute::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_cd40_helm_template_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_helm_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::cd40::helm::template::__export_cd40_helm_template_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::cd40::helm::template);
    };
}
#[doc(inline)]
pub(crate) use __export_helm_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:cd40:helm:helm:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 207] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07U\x01A\x02\x01A\x02\x01\
B\x04\x01p}\x01ps\x01@\x01\x09chart-zip\0\0\x01\x04\0\x07execute\x01\x02\x04\x01\
\x12cd40:helm/template\x05\0\x04\x01\x0ecd40:helm/helm\x04\0\x0b\x0a\x01\0\x04he\
lm\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10\
wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
