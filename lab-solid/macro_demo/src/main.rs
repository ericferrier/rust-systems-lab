use macro_demo::handler;

// ----------------------------
// "Anchor-style program module"
// ----------------------------

#[handler]
fn initialize() {
    println!("Initializing system...");
}

#[handler]
fn transfer() {
    println!("Transferring funds...");
}

fn main() {
    initialize();
    transfer();
}