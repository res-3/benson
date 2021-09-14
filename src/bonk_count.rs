use tracing::info;

const BONK_COUNT_FILE: &str = ".bonk_count";

pub fn add_bonk() {
    info!("Adding another bonk to the count");

    // Read the count
    let count = read_bonk_count() + 1;

    // Write the count again
    std::fs::write(BONK_COUNT_FILE, count.to_string()).unwrap();
}

pub fn read_bonk_count() -> u64 {
    info!("Reading the bonk count");
    return std::fs::read_to_string(BONK_COUNT_FILE)
        .unwrap_or("0".to_string())
        .parse()
        .unwrap();
}
