use crate::primitives::address::Address;
use arrayref::array_ref;
use core::mem::{size_of, transmute, transmute_copy};

const TX_LEN: usize = size_of::<Transaction>();
const SIGNATURE_LEN: usize = 96;

#[repr(packed)]
pub struct Transaction {
    pub to: Address,
    pub from: Address,
    pub nonce: u64,
    pub amount: u64,
    pub signature: [u8; SIGNATURE_LEN],
}

impl Transaction {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        debug_assert_eq!(bytes.len(), TX_LEN);
        unsafe { transmute_copy(array_ref![bytes, 0, TX_LEN]) }
    }

    pub fn to_bytes(self) -> [u8; TX_LEN] {
        unsafe { transmute(self) }
    }

    pub const fn len(&self) -> usize {
        size_of::<Transaction>()
    }
}
