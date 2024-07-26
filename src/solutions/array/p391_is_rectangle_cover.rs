use crate::solutions::Solution;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut total_area = 0;
        let mut points = std::collections::HashSet::new();

        for rect in rectangles.iter() {
            let (x1, y1, x2, y2) = (rect[0], rect[1], rect[2], rect[3]);

            min_x = min_x.min(x1);
            min_y = min_y.min(y1);
            max_x = max_x.max(x2);
            max_y = max_y.max(y2);

            total_area += (x2 - x1) * (y2 - y1);

            let corners = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)];
            for &corner in &corners {
                if !points.insert(corner) {
                    points.remove(&corner);
                }
            }
        }

        let expected_corners = [
            (min_x, min_y),
            (min_x, max_y),
            (max_x, min_y),
            (max_x, max_y),
        ];
        for &corner in &expected_corners {
            if !points.contains(&corner) {
                return false;
            }
        }

        points.len() == 4 && total_area == (max_x - min_x) * (max_y - min_y)
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::ArrayStringExt;

    use super::*;

    #[test]
    fn test_is_rectangle_cover() {
        let rectangles = "[[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]".to_matrix();
        assert!(Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn test_is_rectangle_cover_2() {
        let rectangles = "[[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]".to_matrix();
        assert!(!Solution::is_rectangle_cover(rectangles));
    }

    #[test]
    fn test_is_rectangle_cover_3() {
        let rectangles = "[[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]".to_matrix();
        assert!(!Solution::is_rectangle_cover(rectangles));
    }
}
