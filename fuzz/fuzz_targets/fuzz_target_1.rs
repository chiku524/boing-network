#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }
    let _ = bincode::deserialize::<boing_primitives::Block>(data);
    let _ = bincode::deserialize::<boing_primitives::Transaction>(data);
});
