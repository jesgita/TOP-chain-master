use crate::access::{AccessTuple, EIP_1559_TYPE_BYTE};
use engine_precompiles::secp256k1::ecrecover;
use engine_types::types::{Address, Wei};
use engine_types::{Vec, U256};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Transaction1559 {
    pub chain_id: u64,
    pub nonce: U256,
    pub max_priority_fee_per_gas: U256,
    pub max_fee_per_gas: U256,
    pub gas_limit: U256,
    pub to: Option<Address>,
    pub value: Wei,
    pub data: Vec<u8>,
    pub access_list: Vec<AccessTuple>,
}

impl Transaction1559 {
    /// RLP encoding of the data for an unsigned message (used to make signature)
    pub fn rlp_append_unsigned(&self, s: &mut RlpStream) {
        self.rlp_append(s, 9);
    }

    /// RLP encoding for a signed message (used to encode the transaction for sending to tx pool)
    pub fn rlp_append_signed(&self, s: &mut RlpStream) {
        self.rlp_append(s, 12);
    }

    fn rlp_append(&self, s: &mut RlpStream, list_len: usize) {
        s.begin_list(list_len);
        s.append(&self.chain_id);
        s.append(&self.nonce);
        s.append(&self.max_priority_fee_per_gas);
        s.append(&self.max_fee_per_gas);
        s.append(&self.gas_limit);
        match self.to.as_ref() {
            None => s.append(&""),
            Some(address) => s.append(&address.raw()),
        };
        s.append(&self.value.raw());
        s.append(&self.data);
        s.begin_list(self.access_list.len());
        for tuple in self.access_list.iter() {
            s.begin_list(2);
            s.append(&tuple.address);
            s.begin_list(tuple.storage_keys.len());
            for key in tuple.storage_keys.iter() {
                s.append(key);
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SignedTransaction1559 {
    pub transaction: Transaction1559,
    /// The parity (0 for even, 1 for odd) of the y-value of a secp256k1 signature.
    pub parity: u8,
    pub r: U256,
    pub s: U256,
}

impl SignedTransaction1559 {
    pub fn sender(&self) -> Option<Address> {
        let mut rlp_stream = RlpStream::new();
        rlp_stream.append(&EIP_1559_TYPE_BYTE);
        self.transaction.rlp_append_unsigned(&mut rlp_stream);
        let message_hash = engine_sdk::keccak(rlp_stream.as_raw());
        ecrecover(
            message_hash,
            &super::vrs_to_arr(self.parity, self.r, self.s),
        )
        .ok()
    }
}

impl Encodable for SignedTransaction1559 {
    fn rlp_append(&self, s: &mut RlpStream) {
        self.transaction.rlp_append_signed(s);
        s.append(&self.parity);
        s.append(&self.r);
        s.append(&self.s);
    }
}

impl Decodable for SignedTransaction1559 {
    fn decode(rlp: &Rlp<'_>) -> Result<Self, DecoderError> {
        if rlp.item_count() != Ok(12) {
            return Err(rlp::DecoderError::RlpIncorrectListLen);
        }
        let chain_id = rlp.val_at(0)?;
        let nonce = rlp.val_at(1)?;
        let max_priority_fee_per_gas = rlp.val_at(2)?;
        let max_fee_per_gas = rlp.val_at(3)?;
        let gas_limit = rlp.val_at(4)?;
        let to = super::rlp_extract_to(rlp, 5)?;
        let value = Wei::new(rlp.val_at(6)?);
        let data = rlp.val_at(7)?;
        let access_list = rlp.list_at(8)?;
        let parity = rlp.val_at(9)?;
        let r = rlp.val_at(10)?;
        let s = rlp.val_at(11)?;
        Ok(Self {
            transaction: Transaction1559 {
                chain_id,
                nonce,
                max_priority_fee_per_gas,
                max_fee_per_gas,
                gas_limit,
                to,
                value,
                data,
                access_list,
            },
            parity,
            r,
            s,
        })
    }
}
