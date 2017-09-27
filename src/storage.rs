use hash::H256;

#[derive(Debug)]
pub struct Error;

#[cfg_attr(not(feature="std"), link(name = "env"))]
extern {
    fn storage_read(key: *const u8, dst: *mut u8) -> i32;
    fn storage_write(key: *const u8, src: *const u8) -> i32;
}
/// Performs read from storage
/// Can return `Error` if data is read from outside of the storage boundaries
pub fn read(key: &H256) -> Result<[u8; 32], Error> {
    let mut dst = [0u8; 32];
    match unsafe {
        storage_read(key.as_ptr(), (&mut dst).as_mut_ptr())
    } {
        x if x < 0 => Err(Error),
        _ => Ok(dst),
    }
}

/// Performs write to the storage
pub fn write(key: &H256, val: &[u8; 32]) -> Result<(), Error> {
    match unsafe {
        storage_write(key.as_ptr(), val.as_ptr())
    } {
        x if x < 0 => Err(Error),
        _ => Ok(()),
    }
}
