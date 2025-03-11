const unsafe fn as_slice<T, U>(slice: &[U]) -> Option<&[T]> {
    if slice.len() % size_of::<T>() != 0 {
        return None;
    }

    Some(core::slice::from_raw_parts(
        slice.as_ptr() as *const T,
        slice.len() / size_of::<T>(),
    ))
}

pub const fn as_u8_slice<T>(slice: &[T]) -> Option<&[u8]> {
    unsafe { as_slice(slice) }
}

pub const fn as_u16_slice<T>(slice: &[T]) -> Option<&[u16]> {
    unsafe { as_slice(slice) }
}

pub const fn as_u32_slice<T>(slice: &[T]) -> Option<&[u32]> {
    unsafe { as_slice(slice) }
}

pub const fn as_u64_slice<T>(slice: &[T]) -> Option<&[u64]> {
    unsafe { as_slice(slice) }
}

pub const fn as_u128_slice<T>(slice: &[T]) -> Option<&[u128]> {
    unsafe { as_slice(slice) }
}
