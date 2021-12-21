pub mod poseidon;
pub mod merkle;

use halo2::pasta::Fp;
use crate::merkle::IncrementalTree;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


#[wasm_bindgen]
pub struct IncrementalMerkleTree {
    tree: IncrementalTree
}

#[wasm_bindgen]
impl IncrementalMerkleTree {
    #[wasm_bindgen]
    pub fn new(zero_value: u64, depth: usize) -> Self {
        let z_value_to_field = Fp::from(zero_value);
        IncrementalMerkleTree {
            tree: IncrementalTree::new(z_value_to_field, depth)
        }
    }

    #[wasm_bindgen]
    pub fn insert(leaf: u64) {
        
    }

}