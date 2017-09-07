// use hash::H256;
use bigint::U256;
use hash::Address;

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

        // Various call variants

        /// Direct/classic call.
        /// Correspond to "CALL" opcode in EVM
        pub fn ccall(
            address: *const u8,
            val_ptr: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;

        /// Delegate call.
        /// Corresponds to "CALLCODE" opcode in EVM
        pub fn dcall(
            address: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;

        /// Static call.
        /// Corresponds to "STACICCALL" opcode in EVM
        pub fn scall(
            address: *const u8,
            input_ptr: *const u8,
            input_len: u32,
            result_ptr: *mut u8,
            result_len: u32,
        ) -> i32;

        // enviromental blockchain functions (runtume might not provide all of these!)

        /// Block hash of the specific block
        pub fn blockhash(number: i64, dest: *mut u8) -> i32;

        pub fn coinbase(dest: *mut u8);

        pub fn timestamp() -> i64;

        pub fn blocknumber() -> i64;

        pub fn difficulty(dest: *mut u8);

        pub fn gaslimit(dest: *mut u8);
    }
}

pub fn suicide(refund: &Address) {
    unsafe { external::suicide(refund.as_ptr()); }
}

pub fn create(endowment: U256, code: &[u8]) -> Result<Address, Error> {
    let mut endowment_arr = [0u8; 32];
    endowment.to_big_endian(&mut endowment_arr);
    let mut result = Address::new();
    unsafe {
        if external::create(endowment_arr.as_ptr(), code.as_ptr(), code.len() as u32, (&mut result).as_mut_ptr()) == 0 {
            Ok(result)
        } else {
            Err(Error)
        }
    }
}

pub fn call(address: &Address, value: U256, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    let mut value_arr = [0u8; 32];
    value.to_big_endian(&mut value_arr);
    unsafe {
        match external::ccall(address.as_ptr(), value_arr.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn call_code(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::dcall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn static_call(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::scall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

pub fn block_hash(block_number: u64) -> Result<[u8; 32], Error> {
    let mut res = [0u8; 32];
    unsafe {
        match external::blockhash(block_number as i64, res.as_mut_ptr()) {
            0 => Ok(res),
            _ => Err(Error),
        }
    }
}

pub fn coinbase() -> Address {
    let mut res = Address::new();
    unsafe { external::coinbase(res.as_mut_ptr()); }
    res
}

pub fn timestamp() -> u64 {
    unsafe { external::timestamp() as u64 }
}

pub fn block_number() -> u64 {
    unsafe { external::blocknumber()  as u64 }
}

pub fn difficulty() -> U256 {
    let mut res = [0u8; 32];
    unsafe { external::difficulty(res.as_mut_ptr()); }
    U256::from_big_endian(&res)
}

pub fn gas_limit() -> U256 {
    let mut res = [0u8; 32];
    unsafe { external::gaslimit(res.as_mut_ptr()); }
    U256::from_big_endian(&res)
}
