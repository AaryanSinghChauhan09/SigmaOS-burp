//! SigmaOS-burp — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_burp_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-burp: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-burp: ok");
}
