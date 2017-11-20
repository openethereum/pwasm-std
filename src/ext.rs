use hash::H256;
use bigint::U256;
use hash::Address;

/// Generic wasm error
#[derive(Debug)]
pub struct Error;

mod external {

    #[cfg_attr(not(feature="std"), link(name = "env"))]
    extern {
        pub fn suicide(refund: *const u8) -> !;
    }

    #[cfg_attr(not(feature="std"), link(name = "env"))]
    extern {
        pub fn create(endowment: *const u8, code_ptr: *const u8, code_len: u32, result_ptr: *mut u8) -> i32;
    }

    #[cfg_attr(not(feature="std"), link(name = "env"))]
    extern "C" {
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

        pub fn balance(address: *const u8, dest: *mut u8);

        pub fn coinbase(dest: *mut u8);

        pub fn timestamp() -> i64;

        pub fn blocknumber() -> i64;

        pub fn difficulty(dest: *mut u8);

        pub fn gaslimit(dest: *mut u8);

        pub fn sender(dest: *mut u8);

        pub fn address(dest: *mut u8);

        pub fn value(dest: *mut u8);

        pub fn origin(dest: *mut u8);

        pub fn elog(topic_ptr: *const u8, topic_count: u32, data_ptr: *const u8, data_len: u32);
    }
}

/// Suicide
pub fn suicide(refund: &Address) -> ! {
    unsafe { external::suicide(refund.as_ptr()); }
}

/// Balance
pub fn balance(address: &Address) -> U256 {
    unsafe { fetch_u256(|x| external::balance(address.as_ptr(), x) ) }
}

/// Create
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

/// Call
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

/// Call code
pub fn call_code(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::dcall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

/// Static call
pub fn static_call(address: &Address, input: &[u8], result: &mut [u8]) -> Result<(), Error> {
    unsafe {
        match external::scall(address.as_ptr(), input.as_ptr(), input.len() as u32, result.as_mut_ptr(), result.len() as u32) {
            0 => Ok(()),
            _ => Err(Error),
        }
    }
}

/// Block hash
pub fn block_hash(block_number: u64) -> Result<H256, Error> {
    let mut res = H256::zero();
    unsafe {
        match external::blockhash(block_number as i64, res.as_mut_ptr()) {
            0 => Ok(res),
            _ => Err(Error),
        }
    }
}

unsafe fn fetch_address<F>(f: F) -> Address where F: Fn(*mut u8) {
    let mut res = Address::zero();
    f(res.as_mut_ptr());
    res
}

unsafe fn fetch_u256<F>(f: F) -> U256 where F: Fn(*mut u8) {
    let mut res = [0u8; 32];
    f(res.as_mut_ptr());
    U256::from_big_endian(&res)
}

/// Coinbase
pub fn coinbase() -> Address {
    unsafe { fetch_address(|x| external::coinbase(x) ) }
}

/// Timestamp
pub fn timestamp() -> u64 {
    unsafe { external::timestamp() as u64 }
}

/// Block number
pub fn block_number() -> u64 {
    unsafe { external::blocknumber()  as u64 }
}

/// Difficulty
pub fn difficulty() -> U256 {
    unsafe { fetch_u256(|x| external::difficulty(x) ) }
}

/// Gas limit
pub fn gas_limit() -> U256 {
    unsafe { fetch_u256(|x| external::gaslimit(x) ) }
}

/// Sender
pub fn sender() -> Address {
    unsafe { fetch_address(|x| external::sender(x) ) }
}

/// Origin
pub fn origin() -> Address {
    unsafe { fetch_address(|x| external::origin(x) ) }
}

/// Value
pub fn value() -> U256 {
    unsafe { fetch_u256(|x| external::value(x) ) }
}

/// Address
pub fn address() -> Address {
    unsafe { fetch_address(|x| external::address(x) ) }
}

/// Log
pub fn log(topics: &[H256], data: &[u8]) {
    unsafe { external::elog(topics.as_ptr() as *const u8, topics.len() as u32, data.as_ptr(), data.len() as u32); }
}