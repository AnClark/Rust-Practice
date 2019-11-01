use rand::Rng;

const MAX_NUM: i32 = 100;
const COUNT: usize = 60;

fn main() {
    let mut nums: [i32; COUNT] = [0; COUNT];

    // Generate random numbers
    for num in nums.iter_mut() {
        *num = rand::thread_rng().gen_range(1, MAX_NUM);
    }

    println!("Array to be sorted:");
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!("");

    // Bubble sorting
    for i in (0..COUNT) {
        for j in (i + 1..COUNT) {
            if nums[i] > nums[j] {
                let buf = nums[i];
                nums[i] = nums[j];
                nums[j] = buf;
            }
        }
    }

    println!("Sorted:");
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!("");
}
