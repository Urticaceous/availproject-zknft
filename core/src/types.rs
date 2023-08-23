use risc0_zkvm::sha::rust_crypto::{Digest, Sha256};
#[cfg(feature = "native")]
use risc0_zkvm::SessionReceipt;
use serde::{Deserialize, Serialize};
use sparse_merkle_tree::{traits::Hasher, MerkleProof, H256};

#[derive(Default)]
pub struct ShaHasher(pub Sha256);

impl ShaHasher {
    pub fn new() -> Self {
        ShaHasher(Sha256::new())
    }
}
impl Hasher for ShaHasher {
    fn write_h256(&mut self, h: &H256) {
        self.0.update(h.as_slice())
    }

    fn write_byte(&mut self, b: u8) {
        self.0.update([b])
    }

    fn finish(self) -> H256 {
        let bytes = self.0.finalize();
        let sha2_array: [u8; 32] = bytes.as_slice().try_into().unwrap();
        H256::from(sha2_array)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Commit {
    pub pre_state_root: String,
    pub post_state_root: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StateUpdate<S> {
    pub pre_state_root: H256,
    pub post_state_root: H256,
    pub pre_state_with_proof: (Vec<S>, MerkleProof),
    pub post_state_with_proof: (Vec<S>, MerkleProof),
}

#[cfg(feature = "native")]
#[derive(Debug, Deserialize, Serialize)]
pub struct BatchHeader {
    pub pre_state_root: H256,
    pub state_root: H256, 
    pub transactions_root: H256,
    pub batch_number: u64, 
    pub receipt: SessionReceipt
}

#[cfg(feature = "native")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Batch<T> {
    pub header: BatchHeader,
    pub transactions: Vec<T>
}