use crate::solutions::Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        const N: usize = 30;
        fn ones_positions(img: &[Vec<i32>]) -> Vec<(i32, i32)> {
            let mut ones = vec![];
            #[allow(clippy::needless_range_loop)]
            for i in 0..img.len() {
                for j in 0..img[i].len() {
                    if img[i][j] == 1 {
                        ones.push((i as i32, j as i32));
                    }
                }
            }
            ones
        }

        let target = ones_positions(&img2);
        let mut max_overlap = 0;
        let mut translation_map = vec![vec![0; 2 * N]; 2 * N];
        for (i, j) in ones_positions(&img1) {
            for (x, y) in &target {
                let dx = x - i + N as i32;
                let dy = y - j + N as i32;
                let val = &mut translation_map[dx as usize][dy as usize];
                *val += 1;
                max_overlap = max_overlap.max(*val);
            }
        }
        max_overlap
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_largest_overlap() {
        let img1 = "[[1,1,0],[0,1,0],[0,1,0]]".to_matrix();
        let img2 = "[[0,0,0],[0,1,1],[0,0,1]]".to_matrix();
        assert_eq!(Solution::largest_overlap(img1, img2), 3);
    }

    #[test]
    fn test_largest_overlap_2() {
        let img1 = "[[1]]".to_matrix();
        let img2 = "[[1]]".to_matrix();
        assert_eq!(Solution::largest_overlap(img1, img2), 1);
    }

    #[test]
    fn test_largest_overlap_3() {
        let img1 = "[[0]]".to_matrix();
        let img2 = "[[0]]".to_matrix();
        assert_eq!(Solution::largest_overlap(img1, img2), 0);
    }
}
