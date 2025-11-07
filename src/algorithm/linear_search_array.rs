#[allow(dead_code)]
pub fn linear_search_array(haystack: &[i32], needle: i32) -> bool {
    // return false;

    for item in haystack {
        if *item == needle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::algorithm::linear_search_array::linear_search_array;
    use rstest::{fixture, rstest};

    #[fixture]
    pub fn haystack() -> [i32; 11] {
        [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420]
    }

    #[rstest]
    fn should_find_1(haystack: [i32; 11]) {
        assert_eq!(linear_search_array(&haystack, 1), true)
    }

    #[rstest]
    #[case(0, false)]
    #[case(1, true)]
    #[case(69, true)]
    #[case(1336, false)]
    #[case(69420, true)]
    #[case(69421, false)]
    #[case(1, true)]
    #[case(0, false)]
    #[trace]
    fn parameterized_test(haystack: [i32; 11], #[case] needle: i32, #[case] expected: bool) {
        assert_eq!(linear_search_array(&haystack, needle), expected)
    }
}
