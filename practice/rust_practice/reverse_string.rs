fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    let original = "rust programming";
    let reversed = reverse_string(original);

    println!("{reversed}");
}
