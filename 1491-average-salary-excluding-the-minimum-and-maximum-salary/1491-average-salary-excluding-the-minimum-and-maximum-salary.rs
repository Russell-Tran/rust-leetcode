impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        // find the max: O(n)
        // find the min: O(n)
        // sum and average, skipping the indices of the max and min
        
        let min = f64::from(*salary.iter().min().unwrap());
        let max = f64::from(*salary.iter().max().unwrap());
        let mut sum:f64 = salary.iter().map(|&v| v as f64).sum();
        sum -= max;
        sum -= min;
        let avg = sum / (salary.len() as f64 - 2f64);
        return avg;
    }  
}