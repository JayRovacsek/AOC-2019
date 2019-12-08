use super::*;

#[test]
fn test_part_a() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(7_265_618, run_interpreter(INPUT_VEC.to_vec(), 1));
}

// Tests to ensure input is equal to 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_1() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_ne!(
        1,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 999)
    );
}


// Tests to ensure input is less than 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_2() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 7)
    );
    assert_eq!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 0)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 10)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 6)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 2)
    );
    assert_ne!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 99)
    );
}

// Tests to ensure input is equal to 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_3() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 8)
    );
    assert_ne!(
        1,
        run_interpreter([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 999)
    );
}

// Tests to ensure input is less than 8: on true return 1,
// else return 0
#[test]
fn test_part_b_example_4() {
    assert_eq!(true, true);
    assert_ne!(true, false);
    assert_eq!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 7)
    );
    assert_eq!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 0)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 9)
    );
    assert_eq!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 10)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 6)
    );
    assert_ne!(
        0,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 2)
    );
    assert_ne!(
        1,
        run_interpreter([3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(), 99)
    );
}