#[allow(dead_code)]
pub fn binary_search_array(haystack: &[i32], needle: i32) -> bool {
    let mut low = 0usize;
    let mut high = haystack.len();

    while low < high {
        let position = ((high - low) / 2) + low;
        if haystack[position] == needle {
            return true;
        } else if haystack[position] < needle {
            low = position + 1
        } else {
            high = position
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

// FUCKING WINDOWS WON'T BUILD THIS GOD DAMN PROPERTY TESTING
// #[cfg(test)]
// mod property_test {
//     use crate::algorithm::binary_search_algorithm::binary_search_array;
//     use prop_test::proptest;
//     use prop_test::proptest::proptest;
// 
//     // Property testing my method!
//     proptest! {
//         #[test]
//         fn property_testing(
//             mut generated_vector in proptest::collection::vec(-1000..1000, 0..255),
//             random_index in 0..255usize ){
// 
//             generated_vector.sort()
//             // assert_eq!(true, true);
//             assert_eq!(binary_search_array(&generated_vector, generated_vector[random_index]), true)
//         }
//     }
// }
