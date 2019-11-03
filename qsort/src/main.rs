// Quick sort with Rust
// 
// Reference: https://blog.csdn.net/piedaocaoren/article/details/90340531

use rand::Rng;

const COUNT: usize = 100;
const MAX_NUM: u64 = 1000;

fn main()
{
    // Currently we use vector instead of array
    let mut nums = Vec::new();

    // Generate random numbers to be sorted
    for _i in (0..COUNT) {
        nums.push(
            rand::thread_rng().gen_range(1, MAX_NUM + 1)
        );
    }

    println!("Current num:");
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!("");


    let len = nums.len();
    qsort(&mut nums, 0, len - 1);
    //quick_sort(&mut nums, 0, len - 1);


    println!("Sorted num:");
    for num in nums.iter() {
        print!("{} ", num);
    }
    println!("");
}


fn qsort(arr: &mut Vec<u64>, left: usize, right: usize) {
    if left >= right {
        return
    }

    let pivot = arr[left];          // Select a pivot
    let mut l = left;               // Left pointer
    let mut r = right;              // Right pointer
 
    while l < r {
        while l < r && arr[r] >= pivot {
            r -= 1;
        }
        while l < r && arr[l] <= pivot {
            l += 1;
        }

        arr.swap(l, r);
    }
    
    arr.swap(left, l);

    if l > 1 {          // Prevent overflow error
        qsort(arr, left, l - 1);
    }
    qsort(arr, r + 1, right);
}


// CSDN's reference code
//fn quick_sort(nums: &mut Vec<u64>, left: usize, right: usize) {    if left >= right {        return;    }     let mut l = left;    let mut r = right;    while l < r {        while l < r && nums[r] >= nums[left] {            r -= 1;        }        while l < r && nums[l] <= nums[left] {            l += 1;;        }        nums.swap(l, r);    }    nums.swap(left, l);    if l > 1 {        quick_sort(nums, left, l - 1);    }     quick_sort(nums, r + 1, right);}
