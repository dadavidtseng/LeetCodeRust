//----------------------------------------------------------------------------------------------------
// rust_0001_two_sum.rs
//----------------------------------------------------------------------------------------------------

//----------------------------------------------------------------------------------------------------
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = map.get(&complement) {
            return vec![index as i32, i as i32];
        }
        map.insert(num, i);
    }

    // We assume there is exactly one solution as per the problem statement
    unreachable!()
}

fn main() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;
    let case1 = two_sum(nums1, target1);

    println!("Case1");
    println!("result: {:?}", case1);

    let nums2 = vec![3,2,4];
    let target2 =6;
    let case2 = two_sum(nums2, target2);

    println!("Case2");
    println!("result: {:?}", case2);

    let nums3 = vec![3,3];
    let target3 = 6;
    let case3 = two_sum(nums3, target3);

    println!("Case3");
    println!("result: {:?}", case3);
}
