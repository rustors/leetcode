fn main() {
    // assert_eq!(2, Solution::find_complement(5));
    println!("{:#b}", Solution::find_complement(5));
    println!("{:#b}", Solution::find_complement(1));
    println!("{:#b}", Solution::find_complement(2));
}

struct Solution;

impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let lz = num.leading_zeros();
        !num << lz >> lz
    }
}
