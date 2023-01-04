impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut count:i32 = 0;
        let mut curr:u32 = n;
        while curr > 0 {
            // Take a 1 bit and AND it. If the AND is 1, add to the count.
            // Then bit shift right
            if curr & 1 > 0 {
                count += 1;
            }
            curr = curr >> 1; 
        }
        return count;
    }
}

/*
Alternative potential solution:
// take the log_2 of the integer. use the ceiling as the starting power of 2.
// then descend the powers of 2 to see what fits the bucket
let mut count:i32 = 0;
let mut curr:u32 = n;
let mut power:u32 = 1;
while curr > 0 {
    ...
}
return count;
*/