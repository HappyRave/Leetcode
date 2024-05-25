use crate::solutions::Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        fn backtrack(
            books: &Vec<Vec<i32>>,
            shelf_width: i32,
            idx: usize,
            memo: &mut Vec<i32>,
        ) -> i32 {
            if idx == books.len() {
                return 0;
            }
            if memo[idx] != -1 {
                return memo[idx];
            }
            let mut width = 0;
            let mut height = 0;
            let mut min_height = std::i32::MAX;
            for i in idx..books.len() {
                width += books[i][0];
                if width > shelf_width {
                    break;
                }
                height = height.max(books[i][1]);
                min_height = min_height.min(height + backtrack(books, shelf_width, i + 1, memo));
            }
            memo[idx] = min_height;
            min_height
        }
        let mut memo = vec![-1; books.len()];
        backtrack(&books, shelf_width, 0, &mut memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_height_shelves() {
        assert_eq!(
            Solution::min_height_shelves(
                vec![
                    vec![1, 1],
                    vec![2, 3],
                    vec![2, 3],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 1],
                    vec![1, 2]
                ],
                4
            ),
            6
        );
    }
}
