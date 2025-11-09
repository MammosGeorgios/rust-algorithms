#[allow(dead_code)]
pub fn bubble_sort(mut vec: Vec<i32>) -> Vec<i32> {
    print!("Initial vector is: {:?}", vec);

    for i in 0..vec.len() {
        for j in 0..vec.len() - i - 1 {
            if vec[j] > vec[j + 1] {
                let temp = vec[j + 1];
                vec[j + 1] = vec[j];
                vec[j] = temp;
            }
        }
    }

    // How to print full vector :)
    print!("Result vector is: {:?}", vec);
    vec
}

#[cfg(test)]
mod test {
    use crate::algorithm::bubble_sort::bubble_sort;

    #[test]
    fn single_element_vector_expect_same_vector() {
        let vector = vec![1];
        let sorted_vector = bubble_sort(vector);

        assert_eq!(
            sorted_vector[0] == 1i32,
            true,
            "We expect the same element, without any exceptions"
        )
    }

    #[test]
    fn inverted_vector_expect_is_sorted_true() {
        let vector = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let sorted_vector = bubble_sort(vector);

        assert_eq!(sorted_vector.as_slice().is_sorted(), true)
    }

    #[test]
    fn vector_with_duplicates_expect_is_sorted_true() {
        let vector = vec![9, 9, 9, 9, 8, 8, 19, 32, 32, 8, 7, 6, 5, 4, 3, 2, 1];
        let sorted_vector = bubble_sort(vector);

        assert_eq!(sorted_vector.as_slice().is_sorted(), true)
    }

    #[test]
    fn fem_example_vector_expect_is_sorted_true() {
        let vector = vec![9, 3, 7, 4, 69, 420, 42];
        let sorted_vector = bubble_sort(vector);

        assert_eq!(sorted_vector.as_slice().is_sorted(), true)
    }
}

#[cfg(test)]
mod property_test {
    use crate::algorithm::bubble_sort::bubble_sort;
    use prop_test::proptest;
    use prop_test::proptest::proptest;

    proptest! {
        #[test]
        fn property_test_buttle_sort(
            generated_vector in proptest::collection::vec(i32::MIN..i32::MAX, 0..1000)){

            let sorted_vector = bubble_sort(generated_vector);
            assert_eq!(sorted_vector.as_slice().is_sorted(), true);
        }
    }
}

#[cfg(test)]
mod vector_sorted_test {

    #[test]
    fn vector_is_not_sorted() {
        let unsorted_vector = vec![1, 2, 3, 5, -1];
        assert_ne!(
            unsorted_vector.as_slice().is_sorted(),
            true,
            "The slice.is_sorted() is expected to be false, and not true"
        );
    }

    #[test]
    fn vector_is_sorted() {
        let unsorted_vector = vec![1, 2, 3, 5, 66];
        assert_eq!(unsorted_vector.as_slice().is_sorted(), true);
    }
}
