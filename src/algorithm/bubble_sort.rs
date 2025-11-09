#[cfg(test)]
mod vector_sorted_tests {

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
