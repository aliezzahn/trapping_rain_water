//! # Trapping Rain Water
//!
//! This crate provides a function to calculate the amount of rainwater that can be trapped
//! between the bars of an elevation map represented by a vector of non-negative integers.
//!
//! ## Example
//! ```
//! use trapping_rain_water::trap;
//!
//! let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
//! assert_eq!(trap(height), 6);
//! ```

pub fn trap(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut left_max = 0;
    let mut right_max = 0;
    let mut result = 0;

    while left < right {
        if height[left] < height[right] {
            if height[left] >= left_max {
                left_max = height[left];
            } else {
                result += left_max - height[left];
            }
            left += 1;
        } else {
            if height[right] >= right_max {
                right_max = height[right];
            } else {
                result += right_max - height[right];
            }
            right -= 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        // Test case 1
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);

        // Test case 2
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(trap(height), 9);

        // Test case 3: No water trapped
        let height = vec![1, 2, 3, 4, 5];
        assert_eq!(trap(height), 0);

        // Test case 4: All bars of the same height
        let height = vec![2, 2, 2, 2, 2];
        assert_eq!(trap(height), 0);

        // Test case 5: Single bar
        let height = vec![5];
        assert_eq!(trap(height), 0);
    }
}