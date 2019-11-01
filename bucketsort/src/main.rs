// Bucket sort with Rust
use rand::Rng;

const MAX_NUM: usize = 1000;

fn main() {
    let mut nums = [0; 30];
    let mut buckets = [0; MAX_NUM];

    // Generate numbers to be sorted
    for num in nums.iter_mut() {
        *num = rand::thread_rng().gen_range(1, MAX_NUM + 1);
    }

    println!("current num:");
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!("");


    // Fill in bucket
    for num in nums.iter() {
        buckets[*num] += 1;
    }
    
    // Print result
    println!("Sorted result:");

    let mut i: usize = 0;
    while i < MAX_NUM {
        if buckets[i] > 0 {
            for _k in (0..buckets[i]) {
                print!("{} ", i);
            }
        }

        i += 1;
    }

    println!("");
}
