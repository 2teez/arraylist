
# arraylist

## Name

`arraylist` -- An intutive rust vector wrapper built on top of rust [vector implementation](https://doc.rust-lang.org/std/vec/struct.Vec.html). Taste like Java Arraylist, and can be used like Python list functons and JavaScript array.

## Installation

In the Cargo.toml file

    [dependancies]
    arraylist = {git = "https://github.com/2teez/arraylist"}
        
    OR 
    
    [dependancies]
    arraylist = "0.1.0"

In the main.rs file

### To Use

  ```
 #[macro_use] 
 extern crate arraylist;
 
 use arraylist::arl::ArrayList;
 ```
## Description
`arraylist` - is a rust wrapper built on the rust standard vector implementation. It makes it easier to work more intutively using vector as list or `growable` arrays in other languages like Java, JavaScript and others with little or no "fighting" with rust borrow checker; an ever present, stubborn but great friend coding in rust.

`arraylist` is *_NOT_* a rewrite of `vec` in rust, but rather a wrapping that provides safe, easier and possible interface while using "interior mutability" which rust language made available. 

Using crate `arraylist` makes it possible to work with immutable values, objects etc, yet making several changes without having your code with "mut" keyword everywhere. Though, you might still have to use it in a few cases. But mostly you are working on immutable values.

The functions provided by arraylist crate, bears alot of resemblance with Java ArrayList methods. Even if you have not used java before, it feels intutive and hides some "headache" workings in rust-lang. 

## Example
The code below is not possible just using vec in rust like so:

```
let vec = vec![1, 2, 3, 4, 5];
vec.push(6); // cannot borrow as mutable
```
With arraylist, it works like so, using immutable variable:
```
let arr = arraylist![1, 2, 3, 4];
arr.push(5);
arr.push(6);

// print out your arraylist like so:
arr.print(); // [1, 2, 3, 4, 5, 6]
```
However, to get the same result in `vec`, you must make your variable mutable.

You can also work with mutable variable using `arraylist` like so:
```
let mut arr = arraylist![];
arr.add(1).add(2).add(3).add(4).add(5).finish();

arr.print(); // [1, 2, 3, 4, 5]
// or use
println!("{}", arr); // same result as above
```
Note that when function `finish` is called on the mutable 'object', you get back immutable one. 
You can chain them all up and get an immutable variable.
```
// you can create, add, and assign or print it all out 
// like so

ArrayList::new()                                                        
         .add("lagos")                                                                                                       
         .add("enugu")                                                        
         .add("cairo")                                                        
         .finish()                                                            
         .print();  // ["lagos", "enugu", "cairo"]
// or assign
let places = ArrayList::new().ad(..)..finish();
places.print();
```
Try this one out using vanilla `vec` in rust
```
let al = arraylist![].add("bruno").add("b").add("🦀")
     .add("ß").add("བོད་སྐད་ལ").finish();
     
 // a for_each macro, takes an immutable variable and a closure
 // each of the string in the arraylist becomes a titled case
 for_each!(al, |a| {
     let mut upper = a.chars().collect::<Vec<_>>();
     upper[0] = upper[0].to_ascii_uppercase();
     upper.iter().fold(String::from(""), |mut a, b| {
         a.push(*b);
         a
     })
 })
 .print();  // ["Bruno", "B", "🦀", "ß", "བ\u{f7c}ད་ས\u{f90}ད་ལ"]
 ```
 ### ArrayList Methods
 `arraylist` uses a common name of function/method used in languages like Java, JavaScript, Python and the rest. So, the methods presented below are intutive to use. I intend to both present and demonstrate how each method can be used. But note that, each programmer's expression is limited by the amount of knowledge available to h(im|er).
 
 Below are the list of the available methods in the crate `ArrayList`:
 1. add
 > ***pub fn add(&mut self) -> &ArrayList\<T>***
 > - Pushes a value into a mutable instance of either an empty or non-empty arraylist ArrayList<T>. The length of the list is increased by 1.
 
        
        let al = ArrayList::new()
                    .add("Lagos")
                    .add("Abuja")
                    .finish();
    
    
 2. add_all
 > ***pub fn add_all(&self, collection: &[T])***
 > - Takes a slice refrence, and append each of it's elements to the end of the list. Using the extend function of the underlaying `vec`. The length of the list increased by the number of the elements in the slice. The capacity of the list is also adjusted accordingly.
 
    
    let al = arraylist![1, 2];
        al.add_all(&[3, 4, 5]);
        al.print(); // [1, 2, 3, 4, 5]
    
 3. add_all_at_index
 >***pub fn add_all_at_index(&self, idx: usize, collection: &[T])***
 > - This function takes two parameters; the starting index for the collection to be inserted. And the a slice refrence to be inserted. It pushes the values in the list to right as it adds values from the slice parameter. 
 Note that the starting index *_MUST_* not be greater than the length of the list itself. If the starting index is greater than the length or size of the list, your code panics. 
 
    let al = arraylist![1, 2, 3];
        al.add_all_at_index(1, &[4, 5, 6]);
        al.print(); // [1, 4, 5, 6, 2, 3]
 
 4. cap
 >***pub fn cap(&self) -> usize***
 > - Return the capacity of the list.
 
        println!(
            "{}",
            ArrayList::start_with(&["coke", "fanta", "pepsi", "chapman"]).cap()
         );
 
 5. clear
 >***pub fn clear(&self)***
 > - Clears: it is remove all the elements in the list. 
 
    let al = arraylist![1, 2, 3];
        al.clear();
        al.print(); // []
 
 6. clone
 >***pub fn clone(&self) -> ArrayList\<T>***
 >- Returns a new different ArrayList instance, having the same elements. It is not a refrence. A change to the element made by clone *_does NOT_* in any way affect the other instance.
 
    let new_clone = al.clone();
        new_clone.push(1);
        new_clone.push(3);
        new_clone.print(); // [1, 3]
        al.print();       // []
 
 7. contains
 >***pub fn contains(&self, value: T) -> bool***
 >- Returns true if the list contains the value of the parameter.
 
    let players = arraylist!["Yekini", "Pele", "Ronaldo", "Messi"];
        println!("{}", players.contains("Yakubu")); // false
        println!("{}", players.contains("Ronaldo")); // true
 
 8. copy
 >***pub fn copy(&self) -> &ArrayList\<T>***
 >- Returns a reference "copy" of the arraylist instance. Any change made one, reflect on the other. This is difference from the `clone` method. Note, you can make several copies of that instance.
 
    let new_copy = al.copy();
        new_copy.push(1);
        new_copy.push(3);
        al.push(0);
        new_copy.print(); // [1, 3, 0]
        al.print();       // [1, 3, 0]
 
 9. default
 >***pub fn default(&self) -> ArrayList\<T>***
 >- Implements the Default traits for ArrayList. Returns a new arraylist instance with default values for each of it's elements.
 
        #[derive(Debug, Clone, PartialEq)]
        struct Person<'a> {
            name: &'a str,
            age: u32,
        }
        let array = ArrayList::<Person>::default();
        array.print(); //   []
        array.push(Person {
            name: "boris",
            age: 23,
        });
        array.print(); //  [Person { name: "boris", age: 23 }
        
 10. ensure_capacity
 >***pub fn ensure_capacity(size: usize) -> ArrayList\<T>***
 >- Construts a new and empty ArrayList<T> with a specified capacity. 
 
    let na: ArrayList<u8> = ArrayList::ensure_capacity(10);
        println!("{}", na.cap()); // 10
        
 11. finish
 12. for_each
 13. from_slice
 14. get
 15. index_in
 16. index_of
 17. index_of_all
 18. insert
 19. is_empty
 20. len
 21. new
 22. pop
 23. print
 24. push
 25. push_on_index
 26. remove
 27. remove_if
 28. replace
 29. size
 30. start_with
 31. sub_list
 32. to_vec

## TODO
[] Finish up the presentation and the documentation. All methods and macros works as intended. 
    
[] Carry out more test and benchmarks for complex scenarios.
