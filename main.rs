fn main() {
    let score = 85;

    // Condition Check
    if score >= 80 {
        println!("Pass! Aapne boht badiya perform kiya.");
    } else if score >= 50 {
        println!("Pass! Aap average hain.");
    } else {
        println!("Fail! Aapko aur mehnat ki zaroorat hai.");
    }

    // Rust feature: if ko direct variable mein save kar sakte hain
    let status = if score >= 50 { "PASSED" } else { "FAILED" };
    println!("Final Status: {}", status);
}
