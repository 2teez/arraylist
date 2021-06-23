#![allow(unused, dead_code)]

use std::cell::{Cell, RefCell};
use std::cmp::PartialEq;
use std::fmt;
use std::iter::IntoIterator;
use std::rc::Rc;

// === MACRO SECTION ===
//
#[macro_export]
macro_rules! arraylist {
    () => {
    {
         let al = ArrayList::default();
         al
    }
    };
    ($($x:expr), *) => {{
        let al = ArrayList::new();
        $(al.push($x);) *
        al
    }};
}

#[macro_export]
macro_rules! remove {
    ($x: expr, $y: expr) => {
        for (ind, val) in $x.to_vec().iter().enumerate() {
            if *val == $y {
                $x.remove(ind);
                break;
            }
        }
    };
}

#[macro_export]
macro_rules! for_each {
    ($x: expr, $y: expr) => {
        ArrayList::start_with(&$x.into_iter().map($y).collect::<Vec<_>>());
    };
}
//
// === END OF MACRO SECTION ===

#[derive(Debug, Copy, Clone, PartialEq)]
struct ArrayListParams {
    count: usize,
    len: usize,
    capacity: usize,
}

impl Default for ArrayListParams {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl ArrayListParams {
    fn new(count: usize, len: usize, capacity: usize) -> Self {
        Self {
            count,
            len,
            capacity,
        }
    }

    fn update(&mut self, count: usize, len: usize, cap: usize) {
        self.count = count;
        self.len = len;
        self.capacity = cap;
    }
}

#[derive(Debug, PartialEq)]
pub struct ArrayList<T> {
    vec: Rc<RefCell<Vec<T>>>,
    count: Cell<ArrayListParams>,
}

impl<T: std::fmt::Debug + Clone + PartialEq> Default for ArrayList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug + Clone + PartialEq> ArrayList<T> {
    pub fn new() -> Self {
        ArrayList {
            vec: Rc::new(RefCell::new(vec![])),
            count: Cell::new(ArrayListParams::new(0, 0, 0)),
        }
    }

    pub fn start_with(collection: &[T]) -> Self {
        ArrayList {
            vec: Rc::new(RefCell::new(collection.to_vec())),
            count: Cell::new(ArrayListParams::new(
                // count, len, capacity
                collection.len(),
                collection.len(),
                collection.to_vec().capacity(),
            )),
        }
    }

    fn update_count(&self) -> ArrayListParams {
        let ncounter = self.vec.borrow().len();
        let mut new_array_list_count = self.count.get();
        new_array_list_count.update(ncounter, ncounter, self.vec.borrow().capacity());
        new_array_list_count
    }

    pub fn push(&self, value: T) -> bool {
        self.vec.borrow_mut().push(value);
        self.count.set(self.update_count());
        true
    }

    pub fn push_on_index(&self, index: usize, value: T) {
        let arr = &self.vec.borrow().clone()[index..].to_vec();
        self.vec.borrow_mut()[index] = value;
        let ori = self.vec.borrow()[..=index].to_vec();
        self.vec.borrow_mut().clear();
        self.vec.borrow_mut().extend(ori);
        self.vec.borrow_mut().extend(arr.clone());
        self.count.set(self.update_count());
    }

    pub fn insert(&self, index: usize, value: T) {
        //self.push_on_index(index, value);
        self.vec.borrow_mut().insert(index, value);
        self.count.set(self.update_count());
    }

    pub fn add_all(&self, collection: &[T]) {
        self.vec.borrow_mut().extend(collection.to_vec());
        self.count.set(self.update_count());
    }

    fn add_all_at_start(&self, collection: &[T]) {
        for (idx, val) in collection.iter().enumerate() {
            self.insert(idx, val.clone())
        }
        self.count.set(self.update_count());
    }

    pub fn add_all_at_index(&self, idx: usize, collection: &[T]) {
        match idx {
            0 => self.add_all_at_start(collection),
            _ => {
                if idx >= self.len() {
                    self.add_all(collection);
                }
                let mut counter = idx;

                for val in collection.iter() {
                    self.insert(counter, val.clone());
                    counter += 1;
                }
            }
        }
        self.count.set(self.update_count());
    }

    pub fn replace(&self, index: usize, value: T) {
        self.vec.borrow_mut()[index] = value;
    }

    pub fn clear(&self) {
        self.vec.borrow_mut().clear();
        self.count.set(ArrayListParams::default());
    }

    pub fn clone(&self) -> Self {
        ArrayList {
            vec: Rc::new(RefCell::new(self.vec.borrow().clone())),
            count: Cell::new(self.count.get()),
        }
    }

    pub fn copy(&self) -> &Self {
        self
    }

    pub fn add(&mut self, value: T) -> &mut Self {
        self.push(value);
        self
    }

    pub fn finish(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            count: Cell::new(self.count.get()),
        }
    }

    pub fn ensure_capacity(size: usize) -> Self {
        Self {
            vec: Rc::new(RefCell::new(Vec::with_capacity(size))),
            count: Cell::new(ArrayListParams::new(0, 0, size)),
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.vec.borrow().contains(value)
    }

    pub fn cap(&self) -> usize {
        self.vec.borrow().capacity()
    }

    pub fn len(&self) -> usize {
        self.vec.borrow().len()
    }

    pub fn size(&self) -> usize {
        self.vec.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.borrow().is_empty()
    }

    pub fn pop(&self) -> Option<T> {
        let result = self.vec.borrow_mut().pop();
        self.count.set(self.update_count());
        result
    }

    pub fn remove(&self, index: usize) -> T {
        let result = self.vec.borrow_mut().remove(index);
        self.count.set(self.update_count());
        result
    }

    pub fn remove_if(&self, f: fn(T) -> bool) {
        let result = self
            .vec
            .borrow_mut()
            .clone()
            .into_iter()
            .filter(|a| !f(a.clone()))
            .collect::<Vec<_>>();
        self.clear();
        self.add_all(&result);
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.vec.borrow().clone().into_iter().collect::<Vec<_>>()
    }

    pub fn from_slice(collection: &[T]) -> Self {
        ArrayList::start_with(collection)
    }

    pub fn for_each(&self, f: fn(T)) {
        let result = self.vec.borrow_mut().clone().into_iter().for_each(f);
    }

    pub fn get(&self, index: usize) -> Option<T> {
        if index > self.vec.borrow().len() {
            return None;
        }
        Some(self.vec.borrow()[index].clone())
    }

    pub fn sub_list(&self, start: usize, stop: usize) -> Option<ArrayList<T>> {
        if !(start <= stop && stop <= self.len()) {
            return None;
        }

        let sub_list = ArrayList::new();
        for ind in start..stop {
            sub_list
                .vec
                .borrow_mut()
                .push(self.vec.borrow()[ind].clone());
        }

        sub_list.count.set(sub_list.update_count());

        Some(sub_list)
    }

    pub fn index_of(&self, value: &T) -> Option<usize> {
        if self.contains(value) {
            for (ind, val) in self.vec.borrow().iter().enumerate() {
                if val == value {
                    return Some(ind);
                }
            }
        }
        None
    }

    pub fn index_of_all(&self, value: T) -> Vec<usize> {
        self.clone()
            .into_iter()
            .enumerate()
            .map(|a| if a.1 == value { a.0 as i32 } else { -1 })
            .filter(|a| *a != -1)
            .map(|a| a as usize)
            .collect::<Vec<_>>()
    }

    pub fn index_in(&self, value: usize) -> Option<T> {
        self.get(value)
    }

    pub fn print(&self) {
        println!("{:?}", self.vec.borrow());
    }
}

impl<T: std::fmt::Display + Clone> fmt::Display for ArrayList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.vec
                .borrow()
                .clone()
                .into_iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T: Clone> IntoIterator for ArrayList<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.borrow().clone().into_iter()
    }
}

#[cfg(test)]
mod tests;
