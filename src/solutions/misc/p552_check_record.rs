use crate::solutions::Solution;

impl Solution {
    const MOD: u32 = 1_000_000_007;
    pub fn check_record(n: i32) -> i32 {
        // start at n = 1: "A" or "P" or "L"
        let mut previous_no_absence_and_ends_present: u32 = 1; // P
        let mut previous_no_absence_and_ends_one_late: u32 = 1; // L
        let mut previous_no_absence_and_ends_two_late: u32 = 0;
        let mut previous_one_absence_and_ends_present_or_absent: u32 = 1; //A
        let mut previous_one_absence_and_ends_one_late: u32 = 0;
        let mut previous_one_absence_and_ends_two_late: u32 = 0;

        (2..=n).for_each(|_| {
            let no_absence_and_ends_present = (previous_no_absence_and_ends_present
                + previous_no_absence_and_ends_one_late
                + previous_no_absence_and_ends_two_late)
                % Self::MOD;
            let no_absence_and_ends_one_late = previous_no_absence_and_ends_present;
            let no_absence_and_ends_two_late = previous_no_absence_and_ends_one_late;
            let one_absence_and_ends_present_or_absent = (no_absence_and_ends_present
                + previous_one_absence_and_ends_present_or_absent
                + previous_one_absence_and_ends_one_late
                + previous_one_absence_and_ends_two_late)
                % Self::MOD;
            let one_absence_and_ends_one_late = previous_one_absence_and_ends_present_or_absent;
            let one_absence_and_ends_two_late = previous_one_absence_and_ends_one_late;

            previous_no_absence_and_ends_present = no_absence_and_ends_present;
            previous_no_absence_and_ends_one_late = no_absence_and_ends_one_late;
            previous_no_absence_and_ends_two_late = no_absence_and_ends_two_late;
            previous_one_absence_and_ends_present_or_absent =
                one_absence_and_ends_present_or_absent;
            previous_one_absence_and_ends_one_late = one_absence_and_ends_one_late;
            previous_one_absence_and_ends_two_late = one_absence_and_ends_two_late;
        });
        (((previous_no_absence_and_ends_present
            + previous_no_absence_and_ends_one_late
            + previous_no_absence_and_ends_two_late)
            % Self::MOD
            + (previous_one_absence_and_ends_present_or_absent
                + previous_one_absence_and_ends_one_late
                + previous_one_absence_and_ends_two_late)
                % Self::MOD)
            % Self::MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_record() {
        assert_eq!(Solution::check_record(1), 3);
    }

    #[test]
    fn test_check_record_2() {
        assert_eq!(Solution::check_record(2), 8);
    }

    #[test]
    fn test_check_record_3() {
        assert_eq!(Solution::check_record(10101), 183236316);
    }
}
