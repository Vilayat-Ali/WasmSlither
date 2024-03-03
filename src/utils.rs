use getrandom::getrandom;
use crate::{ROW_BOUND, COL_BOUND};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn generate_random_points() -> (u32, u32) {
        ({
                let mut buffer = [0u8; 4];
                getrandom(&mut buffer).expect("Failed to generate random number");
                let random_number = u32::from_ne_bytes(buffer) % (ROW_BOUND + 1);
                random_number
            },
            {
                let mut buffer = [0u8; 4];
                getrandom(&mut buffer).expect("Failed to generate random number");
                let random_number = u32::from_ne_bytes(buffer) % (COL_BOUND + 1);
                random_number
        })
} 