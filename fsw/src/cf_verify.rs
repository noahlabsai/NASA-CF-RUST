//! CF Application configuration verification.
//!
//! Translated from: cf_verify.h
//!
//! Compile-time assertions for configuration validation.

// Compile-time assertions for configuration validation
const _: () = {
    // limit number of channels to a reasonable amount
    if crate::cf_platform_cfg::CF_NUM_CHANNELS > 250 {
        panic!("That's a lot of channels. I salute you, but it's too many.");
    }

    if crate::cf_platform_cfg::CF_NUM_CHANNELS == 0 {
        panic!("Must have at least one channel.");
    }

    if crate::cf_platform_cfg::CF_NUM_HISTORIES > 65535 {
        panic!("refactor code for 32 bit CF_NUM_HISTORIES");
    }
};
