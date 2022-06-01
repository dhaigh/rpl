use super::interp::eval_source;

fn equal_vec<T: std::cmp::PartialEq>(a: Vec<T>, b: Vec<T>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for i in 0..a.len() {
        if a[i] != b[i] {
            return false;
        }
    }

    true
}

macro_rules! assert_eq {
    ($a:literal, $b:expr) => {
        assert!(equal_vec(eval_source($a).unwrap(), $b));
    };
}

#[test]
fn test_no_ops() {
    assert_eq!("1", vec![1]);
    assert_eq!("2 3 4", vec![2, 3, 4]);
    assert_eq!("10 5 7", vec![10, 5, 7]);
}

#[test]
fn test_plus() {
    assert_eq!("1 + 1", vec![2]);
    assert_eq!("1 + 1 2", vec![2, 3]);
    assert_eq!("1 2 3 + 1 2 3", vec![2, 4, 6]);
    assert_eq!("9 1 2 + 1", vec![10, 2, 3]);
}

#[test]
fn test_minus() {
    assert_eq!("1 - 1", vec![0]);
    assert_eq!("1 2 - 1 1", vec![0, 1]);
    assert_eq!("1 - 1 2 3", vec![0, -1, -2]);
    assert_eq!("1 2 - 1 2", vec![0, 0]);
    assert_eq!("- 5 6 7", vec![-5, -6, -7]);
}
