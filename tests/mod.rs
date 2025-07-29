#![allow(unused, dead_code)]

use arraylist::{arl::ArrayList, arraylist};

#[test]
pub fn test_arraylist_swapping() {
    let langs = arraylist!["clojure", "golang", "kotlin", "groovy"];
    langs.swap(1, 3);
    langs.print(); // ["clojure", "groovy", "kotlin", "golang"]
    assert_eq!(
        langs.to_vec(),
        ["clojure", "groovy", "kotlin", "golang"].to_vec()
    );
}

#[test]
pub fn test_arraylist_swap() {
    let mut a = arraylist!["java", "kotlin"];
    a.swap(0, 1);
    assert_eq!(a, arraylist!("kotlin", "java"));
}

#[test]
pub fn test_arraylist_swap_comparing_using_numbers() {
    let a = arraylist![3, 9, 0, -1, 4, 5, 10, -3];
    for _ in 0..a.len() {
        for j in 0..a.len() - 1 {
            if a.get(j).unwrap() > a.get(j + 1).unwrap() {
                a.swap(j, j + 1);
            }
        }
    }
    assert_eq!(a.to_vec(), arraylist!(-3, -1, 0, 3, 4, 5, 9, 10).to_vec());
}

#[test]
#[ignore = "todo"]
pub fn test_arraylist_swap_comparing() {
    let a = arraylist!["java", "kotlin", "clojure"];
    for _ in 0..a.len() {
        for j in 0..a.len() - 1 {
            if a.get(j).unwrap() > a.get(j + 1).unwrap() {
                a.swap(j, j + 1);
            }
        }
    }
    assert_eq!(a.to_vec(), arraylist!("clojure", "kotlin", "java").to_vec());
}
