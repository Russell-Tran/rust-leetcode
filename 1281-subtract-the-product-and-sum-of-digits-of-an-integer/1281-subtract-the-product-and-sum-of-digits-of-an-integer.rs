impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let mut prod:i32 = 1;
        let mut sum:i32 = 0;
        let mut curr:i32 = n;
        while curr > 0 {
            let digit = curr % 10;
            prod *= digit;
            sum += digit;
            curr /= 10;
        }
        return prod - sum;
    }
}