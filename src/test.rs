#[cfg(test)]
use crate::can_construct;

#[test]
fn test_1() {
    let result = can_construct(String::from("a"), String::from("b"));
    assert_eq!(result, false);
}

#[test]
fn test_2() {
    let result = can_construct(String::from("aa"), String::from("ab"));
    assert_eq!(result, false);
}

#[test]
fn test_3() {
    let result = can_construct(String::from("aa"), String::from("aab"));
    assert_eq!(result, true);
}

#[test]
fn test_4() {
    let result = can_construct(String::from("aab"), String::from("baa"));
    assert_eq!(result, true);
}

#[test]
fn test_5() {
    let result = can_construct(
        String::from("bg"),
        String::from("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj"),
    );
    assert_eq!(result, true);
}

#[test]
fn test_6() {
    let result = can_construct(
        String::from("fffbfg"),
        String::from("effjfggbffjdgbjjhhdegh"),
    );
    assert_eq!(result, true);
}
