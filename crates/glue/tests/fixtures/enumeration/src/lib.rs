use roc_app;
use roc_std::RocStr;

#[no_mangle]
pub extern "C" fn rust_main() {
    use std::cmp::Ordering;
    use std::collections::hash_set::HashSet;

    let tag_union = roc_app::mainForHost();

    // Verify that it has all the expected traits.

    assert!(tag_union == tag_union); // PartialEq
    assert!(tag_union.clone() == tag_union.clone()); // Clone

    // Since this is a move, later uses of `tag_union` will fail unless `tag_union` has Copy
    let union2 = tag_union; // Copy

    assert!(tag_union.partial_cmp(&tag_union) == Some(Ordering::Equal)); // PartialOrd
    assert!(tag_union.cmp(&tag_union) == Ordering::Equal); // Ord

    let mut set = HashSet::new();

    set.insert(tag_union); // Eq, Hash
    set.insert(union2);

    assert_eq!(set.len(), 1);

    println!(
        "tag_union was: {:?}, Bar is: {:?}, Baz is: {:?}",
        tag_union,
        roc_app::MyEnum::Bar,
        roc_app::MyEnum::Baz,
    ); // Debug
}

// Externs required by roc_std and by the Roc app

use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(msg: *mut RocStr, tag_id: u32) {
    match tag_id {
        0 => {
            eprintln!("Roc standard library hit a panic: {}", &*msg);
        }
        1 => {
            eprintln!("Application hit a panic: {}", &*msg);
        }
        _ => unreachable!(),
    }
    std::process::exit(1);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dbg(loc: *mut RocStr, msg: *mut RocStr) {
    eprintln!("[{}] {}", &*loc, &*msg);
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}
