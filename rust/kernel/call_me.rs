use crate::c_types;

#[no_mangle]
fn rust_called_from_c(ptr: *mut c_types::c_int, size: c_types::c_size_t) -> c_types::c_int {
    let mut sum: c_types::c_int = 0;

    // TODO: why does *(ptr + i) not work / why doies
    for i in 0..size {
        unsafe {
            *ptr = *(ptr) + 1;
            sum += *(ptr)
        }
    }

    sum
}
