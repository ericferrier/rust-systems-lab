✅ Output
[dispatcher] calling handler: initialize
Initializing system...

[dispatcher] calling handler: transfer
Transferring funds...
🧠 What just happened (important)

You wrote:

#[handler]
fn initialize() {}

But Rust expanded it at compile time into something like:

fn initialize() {
    println!("dispatcher...");
    println!("Initializing system...");
}

👉 That is exactly the Anchor philosophy:

“Write high-level declarative code → macros generate low-level plumbing”

⚡ Anchor-style mapping
Anchor concept	Our demo
#[program]	could be module-level macro
#[handler]	function attribute macro
Context<T>	omitted here (would be macro-generated struct)
instruction dispatch	macro expansion
🧠 Key insight

Rust macros let you build:

DSLs (Domain Specific Languages)
frameworks (like Anchor, Serde, Tokio macros)
hidden boilerplate generators
compile-time code rewriting