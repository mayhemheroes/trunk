use honggfuzz::fuzz;
use trunk::common::parse_public_url;

fn main() {
    // Define the fuzzing loop
    loop {
        // Fuzzing input will be provided by Honggfuzz
        fuzz!(|data: &[u8]| {
            // Convert the input to a string
            if let Ok(name) = std::str::from_utf8(data) {
                // Call the parse_public_url function with the fuzzed input
                let _ = parse_public_url(name);

            }
        });
    }
}