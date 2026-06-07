fn build_haystack(target_chars: usize) -> String {
    let mut haystack = String::new();
    while haystack.len() < target_chars {
        haystack.push_str(
            "Photosynthesis is the process by which green plants, algae, and some bacteria convert
light energy into chemical energy stored in sugars.",
        );
    }
    haystack.truncate(target_chars);
    haystack
}

fn plant_needle(haystack: String, needle: &str, depth: f64) -> String {
    let insert_position = (haystack.len() as f64 * depth) as usize;
    let before = &haystack[..insert_position];
    let after = &haystack[insert_position..];
    return format!("{},{},{}", before, needle, after);
}

fn main() {
    let haystack = build_haystack(4000);
    let result = plant_needle(haystack, "The secret code is 4471", 0.5);
    println!("{}", result)
}
