use super::*;

#[test]
fn test_arraylist_new() {
    let alist: ArrayList<u8> = ArrayList::new();
    alist.push(2);
    alist.push(4);
    alist.push(6);
    assert_eq!(
        alist,
        ArrayList {
            vec: Rc::new(RefCell::new(vec![2, 4, 6])),
            count: alist.count.clone()
        }
    );
}

#[test]
fn test_push_on_index() {
    let al: ArrayList<&str> = ArrayList::new();
    al.push("timo");
    al.push("temi");
    al.push_on_index(0, "John");
    assert_eq!(
        al,
        ArrayList {
            vec: Rc::new(RefCell::new(vec!["John", "timo", "temi"])),
            count: al.count.clone()
        }
    );
}

#[test]
#[should_panic(expected = "Different values and counters")]
fn test_add_all() {
    let initial_array = ArrayList::start_with(&[1.5, 3.34, 4.12]);
    let others = vec![0.015, 9.34, 6.12, 99.45];
    initial_array.add_all(&others);
    assert_eq!(
        initial_array,
        ArrayList {
            vec: Rc::new(RefCell::new(vec![1.1, 2.1, 4.2])),
            count: Cell::new(ArrayListParams::new(3, 3, 3))
        }
    );
}

#[test]
fn test_clear() {
    let al = ArrayList::start_with(&vec![1, 3, 5, 7]);
    al.clear();
    assert_eq!(
        al,
        ArrayList {
            vec: Rc::new(RefCell::new(vec![])),
            count: Cell::new(ArrayListParams::default())
        }
    );
}

#[test]
fn test_clone() {
    let mut al = ArrayList::<String>::default();
    al.add("lagos".to_string())
        .add("ibadan".to_string())
        .add("enugu".to_owned())
        .add("abuja".to_owned())
        .finish();
    let copy = al.clone();
    assert_eq!(copy, al);
}

#[test]
fn test_contains() {
    let al = ArrayList::start_with(&['a', 'b', 'x', 'z']);
    assert_eq!(al.contains('a'), true);
}

#[test]
fn test_pop() {
    let mut al = ArrayList::default();
    al.add(3).add(5).add(7).finish();
    assert_eq!(al.pop(), Some(7))
}

#[test]
fn test_remove() {
    let al = ArrayList::<u32>::new();
    al.push(23);
    al.push(25);
    al.push(27);
    al.remove(1);
    assert_eq!(
        al,
        ArrayList {
            vec: Rc::new(RefCell::new(vec![23, 27])),
            count: al.count.clone()
        }
    );
}

#[test]
fn test_get_function() {
    let al = ArrayList::start_with(&[2, 6, 9, 0, 1]);
    assert_eq!(al.get(2), Some(9))
}

#[test]
#[should_panic(expected = "Out of bound index!")]
fn test_get_function_out_of_bound_index() {
    let al = ArrayList::start_with(&[2, 6, 9, 0, 1]);
    assert_eq!(al.get(6), Some(9))
}

#[test]
fn test_get_index_of() {
    let al = ArrayList::start_with(&[2, 6, 9, 0, 1]);
    assert_eq!(al.index_of(0), Some(3));
}

#[test]
fn test_get_index() {
    let al = arraylist!["Africa", "North America", "South America", "Asia", "Europe"];
    assert_eq!(al.index_in(2), Some("South America"));
}

#[test]
fn test_add_all_at_start() {
    let al = arraylist!["C", "cpp"];
    al.add_all_at_start(&["rust", "asm"]);
    assert_eq!(al.to_vec(), vec!["rust", "asm", "C", "cpp"]);
}

#[test]
fn test_add_all_any_location() {
    let al = arraylist![1, 2, 3, 7, 8, 9];
    al.add_all_at_index(3, &[4, 5, 6]);
    assert_eq!(al.to_vec(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
