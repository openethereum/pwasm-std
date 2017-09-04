#[derive(Debug)]
pub struct Error;

mod external {
    #[link(name = "env")]
    extern {
        pub fn suicide(refund: *const u8);
    }
    #[link(name = "env")]
    extern {
        pub fn create(endowment: *const u8, code_ptr: *const u8, code_len: u32, result_ptr: *mut u8) -> i32;
    }
    #[link(name = "env")]
    extern {
        pub fn ccall(
            address: *const u8,
            val_ptr: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;
        pub fn dcall(
            address: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;
        pub fn scall(
            address: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;
    }
}

pub fn suicide(refund: &[u8; 20]) {
    unsafe { external::suicide(refund.as_ptr()); }
}

pub fn create(endowment: &[u8; 32], code: &[u8]) -> Result<[u8; 20], Error> {
    let mut result = [0u8; 20];
    unsafe {
        if external::create(endowment.as_ptr(), code.as_ptr(), code.len() as u32, (&mut result).as_mut_ptr()) == 0 {
            Ok(result)
        } else {
            Err(Error)
        }
    }
}

pub fn call(address: &[u8; 20], value: &[u8; 32], input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::ccall(address.as_ptr(), value.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn call_code(address: &[u8; 20], input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::dcall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn static_call(address: &[u8; 20], input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::scall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn block_hash(block_number: u64) -> Result<[u8; 32], Error> {
}

pub fn coinbase() -> [u8; 20] {
}

pub fn timestamp() -> u64 {
}

pub fn block_number() -> u64 {
}

pub fn difficulty() -> [u8; 32] {
}

pub fn gas_limit() -> [u8; 32] {
}