/*
Two crystal balls problem:
You are in a building with 100 (could be any number) floors, and you have 2 crystal balls.
After a certain floor, if ou drop a ball to the ground, it will break.
Using your two balls, find out which floor is the first one that causes your ball to break.
 */

#[allow(dead_code)]
pub fn two_crystal_balls(breaks_vector: &Vec<bool>) -> i32 {
    let jump_amount: usize = f64::sqrt(breaks_vector.len() as f64) as usize;

    // First split the array in sqrt(N) batches an check the first element in each
    let mut break_position = -1;
    for i in (jump_amount..breaks_vector.len()).step_by(jump_amount) {
        if breaks_vector[i] == true {
            break_position = i as i32;
            break;
        }
    }

    // This means we never found a break point by our square root search
    // We might have leftover in the last set though, so we will look at just the last few entries
    if break_position == -1 {
        break_position = breaks_vector.len() as i32;
    }

    // Loop the picked batch, which has size sqrt(N) as well
    for i in (break_position as usize - jump_amount)..(break_position as usize) {
        if breaks_vector[i] == true {
            return i as i32;
        }
    }

    // Since we have O(sqrt(N) + sqrt(N)), this algorithm is actually O(sqrt(N))
    -1
}

#[cfg(test)]
mod test {
    use crate::algorithm::two_crystal_balls::two_crystal_balls;

    #[test]
    fn all_false_expect_minus_one() {
        let vector = vec![false; 100];
        assert_eq!(
            two_crystal_balls(&vector),
            -1i32,
            "If no floor is causing the ball to break, we expect the default value -1"
        )
    }

    #[test]
    fn all_true_expect_zero() {
        let vector = vec![true; 100];
        assert_eq!(two_crystal_balls(&vector), 0)
    }

    #[test]
    fn test_with_specific_index() {
        let mut vector = vec![false; 10000];

        let first_true_index: i32 = 1221;
        for i in first_true_index as usize..10000 {
            vector[i] = true;
        }

        assert_eq!(
            two_crystal_balls(&vector),
            first_true_index,
            "We expect {first_true_index} as the index"
        )
    }
}
