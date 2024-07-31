use crate::solutions::Solution;

impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let n = s.len();

        let mut prefix_sums = vec![vec![0; n + 1]; 26];
        for (i, c) in s.chars().enumerate() {
            let idx = (c as u8 - b'a') as usize;
            for prefix_sum in &mut prefix_sums {
                prefix_sum[i + 1] = prefix_sum[i];
            }
            prefix_sums[idx][i + 1] += 1;
        }

        let prefix_sums = prefix_sums
            .iter()
            .filter(|prefix_sum| prefix_sum[n] > 0)
            .collect::<Vec<_>>();

        fn is_balanced(prefix_sums: &[&Vec<i32>], start: usize, end: usize) -> bool {
            let mut count = -1;
            for prefix_sum in prefix_sums {
                let freq = prefix_sum[end] - prefix_sum[start];
                if freq > 0 {
                    if count == -1 {
                        count = freq;
                    } else if count != freq {
                        return false;
                    }
                }
            }
            true
        }

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for j in 0..i {
                if is_balanced(&prefix_sums, j, i) {
                    dp[i] = dp[i].min(dp[j] + 1);
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p3144_minimum_substrings_in_partition() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("fabccddg".to_string()), // cspell: disable-line
            3
        );
    }

    #[test]
    fn p3144_minimum_substrings_in_partition_2() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("abababaccddb".to_string()), // cspell: disable-line
            2
        );
    }

    #[test]
    fn p3144_minimum_substrings_in_partition_3() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("ababcc".to_string()), // cspell: disable-line
            1
        );
    }

    #[test]
    fn p3144_minimum_substrings_in_partition_4() {
        assert_eq!(
            Solution::minimum_substrings_in_partition("qvofqcclkcfyahhubjqddfbzccmoqgtmhelitwinnnmzmloznuwuokkbgyighfvhvvsfngsclzzmakkrjiinojjovlkjasdivptbojacjwwwpccwrmmyvczwsmzugijvvzsrktxtjgipnbipywjdzdmrtfnkkaattfeeaghtolifxdwaxgrvidvjdnkocbdsylocbjkashbzzzylcqomfffewyqlisjuerwlvqsymuzmdddmuiylryriqjkxnllmhhqemuvvznbdsxqrblvshgepthhlhemppzohnlllajopgfghhwwywskrzdscmpnvufxxxcygggpgkkdvljouuvynnnxnipvhzaiwcbspmtyyodfxwtryidbcclirnmqgcomoeyyddoonyebboxcilzeegguwdzbecvjoemurddzxxiikijjghkykbfzyknetrsrbmyadwwztsehdwwjzllrwuqojqmneskodjcyuzqmyyrcttrnzrttmhhvqgocpbapidgpuuqgwvzedxccuyzicgpkbbfckeixquuucallgffrvvddokjacvacpqfkmrlomgnaemwaylmszhlcimllmexlzxzxwidofjjvcheltpshddivvyueheiidvsmqwrcbwzddnhkneuxohnpfiunykysmsguwnhfaeqnfpybovxxhfigicocnjndwxhavgkkxongcuexfommwwmrsetbvbdcmxkjppggkpxmlyrfeeelwkuixmbnwwmiipjltzftthnisaeyzgarbfypthwutmbjjrelwrkccwnzwwauubwkoxjbqjqhvcfkfabesswszzmivpgfxtdhlgwaoutpwrzvmahrhyowiywetfrwfxdfycaetphtqhbowfnzvoknyjawvdilrxxivhoihfjqxmmnydddaxkpwwmdblerucsrsrgxionaoakdyxfuagatweyzxxcffiruzrqse".to_string()), // cspell: disable-line
            189
        );
    }
}
