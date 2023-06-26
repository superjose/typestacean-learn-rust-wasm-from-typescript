pub fn rust_syntax() {
    // console.log() Equivalent
    println!("Rust syntax");

    // const map = new Map<String, String>() Equivalent
    let mut map = std::collections::HashMap::new();
    // map.set('key', 'val') Equivalent
    map.insert("key", "value");

    // const arr = [] Equivalent
    let mut arr = vec![];
    // arr.push('val') Equivalent
    arr.push("value");
}
