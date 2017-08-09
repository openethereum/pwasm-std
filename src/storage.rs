pub struct Error;

#[link(name = "env")]
extern {
    fn storage_read(key: *const u8, dst: *mut u8) -> i32;
    fn storage_write(key: *const u8, src: *const u8) -> i32;
}

/// Performs read from storage to the specified slice `dst`
/// Can return `Error` if data is read from outside of the storage boundaries
pub fn read(key: &[u8; 32], dst: &mut [u8; 32]) -> Result<(), Error> {
    match unsafe {
        let mut dst = dst;
        storage_read(key.as_ptr(), dst.as_mut_ptr())
    } {
        x if x < 0 => Err(Error),
        _ => Ok(()),
    }
}

/// Performs write to the storage from the specified `src`
pub fn write(key: &[u8; 32], src: &[u8; 32]) -> Result<(), Error> {
    match unsafe {
        storage_write(key.as_ptr(), src.as_ptr())
    } {
        x if x < 0 => Err(Error),
        _ => Ok(()),
    }
}