Run it
cargo run


🧠 Key TakeAway

System introspection (sysinfo)
CLI loop + refresh
Sorting + iterators
Real-time display
🧪 Sample Output
=== System Monitor ===

CPU Usage: 23.45%
Memory: 4200 MB / 16000 MB

Top Processes:
chrome | CPU: 12.3% | MEM: 500000 KB
rustc  | CPU: 8.1%  | MEM: 200000 KB
node   | CPU: 6.5%  | MEM: 150000 KB

🔥 Easy upgrades (very worth it)
1. Add CLI flags

Use clap:

cargo run -- --interval 2 --top 10