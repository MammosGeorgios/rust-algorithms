#[allow(dead_code)]
pub fn binary_search_array(haystack: &[i32], needle: i32) -> bool {
    let mut low = 0usize;
    let mut high = haystack.len();

    while low < high {
        let middle_point = ((high - low) / 2) + low;
        let value = haystack[middle_point];
        if value == needle {
            return true;
        } else if value < needle {
            low = middle_point + 1
        } else {
            high = middle_point
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::algorithm::binary_search_algorithm::binary_search_array;
    use rstest::{fixture, rstest};

    #[fixture]
    pub fn sorted_haystack() -> [i32; 11] {
        [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420]
    }

    #[rstest]
    fn find_first_value_in_array(sorted_haystack: [i32; 11]) {
        assert_eq!(binary_search_array(&sorted_haystack, 1), true)
    }
    #[rstest]
    fn find_last_value_in_array(sorted_haystack: [i32; 11]) {
        assert_eq!(binary_search_array(&sorted_haystack, 69420), true)
    }

    #[rstest]
    fn find_middle_value_in_array(sorted_haystack: [i32; 11]) {
        assert_eq!(binary_search_array(&sorted_haystack, 81), true)
    }

    #[rstest]
    fn missing_value_in_array(sorted_haystack: [i32; 11]) {
        assert_eq!(binary_search_array(&sorted_haystack, -1), false)
    }

    #[rstest]
    fn single_element_array() {
        let single_element_array = [42];
        assert_eq!(binary_search_array(&single_element_array, 42), true);
    }

    #[rstest]
    #[case(69, true)]
    #[case(1336, false)]
    #[case(69420, true)]
    #[case(69421, false)]
    #[case(1, true)]
    #[case(0, false)]
    #[trace]
    fn tutorial_binary_search_tests(
        sorted_haystack: [i32; 11],
        #[case] needle: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(binary_search_array(&sorted_haystack, needle), expected)
    }
}

#[cfg(test)]
mod property_test {
    use crate::algorithm::binary_search_algorithm::binary_search_array;
    use prop_test::proptest;
    use prop_test::proptest::proptest;

    // Property testing my method!
    proptest! {
        #[test]
        fn property_testing(
            mut generated_vector in proptest::collection::vec(i32::MIN..i32::MAX, 1..10000)){
            generated_vector.sort();
            for i in 0usize..generated_vector.len(){
                assert_eq!(binary_search_array(&generated_vector, generated_vector[i]), true)
                }
        }
    }
}
