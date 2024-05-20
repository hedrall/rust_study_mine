const DEBUG_MODE_KEY: &str = "DEBUG_MODE";
pub fn is_debug_mode() -> bool {
    let debug_mode = std::env::var(DEBUG_MODE_KEY);
    match debug_mode {
        Ok(value) => value == "true",
        Err(_) => false,
    }
}