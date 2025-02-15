use std::thread;

fn main() {
    println!("[KAI Brain] Hi (Ominously)");

    for i in 0..16 {
        println!("[KAI Brain] Loading Brain Segment: 0x{:x}", i);
        thread::sleep(std::time::Duration::from_millis(500));
    }
}