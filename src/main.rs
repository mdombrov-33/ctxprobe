fn build_haystack(target_chars: usize) -> String {
    let mut haystack = String::new();
    while haystack.len() < target_chars {
        haystack.push('a');
    }
    haystack
}
fn main() {
    let result = build_haystack(100);
    println!("{}", result.len())
}
