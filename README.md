# arraylist

## Name

`arraylist` -- An intutive rust vector wrapper built on top of rust [vector implementation](https://doc.rust-lang.org/std/vec/struct.Vec.html). Taste like Java Arraylist, and can be used like Python list functons and JavaScript array.

## Installation

In the Cargo.toml file

    [dependancies]
    arraylist = {git = "https://github.com/2teez/arraylist"}

    OR

    [dependancies]
    arraylist = "0.1.3"

In the main.rs file

### To Use

```
#[macro_use]
extern crate arraylist;

use arraylist::arl::ArrayList;
```

## Description

`arraylist` - is a rust wrapper built on the rust standard vector implementation. It makes it easier to work more intutively using vector as list or `growable` arrays in other languages like Java, JavaScript and others with little or no "fighting" with rust borrow checker; an ever present, stubborn but great friend coding in rust.

`arraylist` is _*NOT*_ a rewrite of `vec` in rust, but rather a wrapping that provides safe, easier and possible interface while using "interior mutability" which rust language made available.

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

1.  add

    > **_pub fn add(&mut self) -> &ArrayList\<T>_**
    >
    > - Pushes a value into a mutable instance of either an empty or non-empty arraylist ArrayList<T>. The length of the list is increased by 1.

           let al = ArrayList::new()
                       .add("Lagos")
                       .add("Abuja")
                       .finish();

2.  add_all

    > **_pub fn add_all(&self, collection: &[T])_**
    >
    > - Takes a slice refrence, and append each of it's elements to the end of the list. Using the extend function of the underlaying `vec`. The length of the list increased by the number of the elements in the slice. The capacity of the list is also adjusted accordingly.

        let al = arraylist![1, 2];
            al.add_all(&[3, 4, 5]);
            al.print(); // [1, 2, 3, 4, 5]

3.  add_all_at_index

    > **_pub fn add_all_at_index(&self, idx: usize, collection: &[T])_**
    >
    > - This function takes two parameters; the starting index for the collection to be inserted. And the a slice refrence to be inserted. It pushes the values in the list to right as it adds values from the slice parameter.
    >   Note that the starting index _*MUST*_ not be greater than the length of the list itself. If the starting index is greater than the length or size of the list, your code panics.

        let al = arraylist![1, 2, 3];
        al.add_all_at_index(1, &[4, 5, 6]);
        al.print(); // [1, 4, 5, 6, 2, 3]

4.  cap

    > **_pub fn cap(&self) -> usize_**
    >
    > - Return the capacity of the list.

           println!(
               "{}",
               ArrayList::start_with(&["coke", "fanta", "pepsi", "chapman"]).cap()
            );

5.  clear

    > **_pub fn clear(&self)_**
    >
    > - Clears: it is remove all the elements in the list.

            let al = arraylist![1, 2, 3];
            al.clear();
            al.print(); // []

6.  clone

    > **_pub fn clone(&self) -> ArrayList\<T>_**
    >
    > - Returns a new different ArrayList instance, having the same elements. It is not a refrence. A change to the element made by clone _*does NOT*_ in any way affect the other instance.

            let new_clone = al.clone();
            new_clone.push(1);
            new_clone.push(3);
            new_clone.print(); // [1, 3]
            al.print(); // []

7.  contains

    > **_pub fn contains(&self, value: T) -> bool_**
    >
    > - Returns true if the list contains the value of the parameter.

            let players = arraylist!["Yekini", "Pele", "Ronaldo", "Messi"];
            println!("{}", players.contains("Yakubu")); // false
            println!("{}", players.contains("Ronaldo")); // true

8.  copy

    > **_pub fn copy(&self) -> &ArrayList\<T>_**
    >
    > - Returns a reference "copy" of the arraylist instance. Any change made one, reflect on the other. This is difference from the `clone` method. Note, you can make several copies of that instance.

                let new_copy = al.copy();
                new_copy.push(1);
                new_copy.push(3);
                al.push(0);
                new_copy.print(); // [1, 3, 0]
                al.print(); // [1, 3, 0]

9.  default

    > **_pub fn default(&self) -> ArrayList\<T>_**
    >
    > - Implements the Default traits for ArrayList. Returns a new arraylist instance with default values for each of it's elements.

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

    > **_pub fn ensure_capacity(size: usize) -> ArrayList\<T>_**
    >
    > - Construts a new and empty ArrayList<T> with a specified capacity.

            let na: ArrayList<u8> = ArrayList::ensure_capacity(10);
            println!("{}", na.cap()); // 10

11. finish

    >**_pub fn finish(&self) -> ArrayList\<T>_**
    >
    >- Returns an immutable `arraylist`. Often called after the use of `add` function, which makes a mutable list and pushes it value into the list. `finish` is the function  to call give immutable variable you wanted.
    
            let al = arraylist![]
                    .add("Lagos")
                    .add("Abuja")
                    .finish();  // immutable 
                al.print();     // ["Lagos", "Abuja"]
    
12. for_each

    >**_pub fn for_each(&self, f: fn(T))_**
    >
    >- It takes a closure and perform action in the closure on each of the elements in the list. But it does *_NOT_* change the element of the list.
    
            // square all odd numbers
            let nums = arraylist!(1, 2, 3, 4, 5);
            nums.for_each(|a| {
                println!(
                    "{}",
                    match a % 2 != 0 {
                        true => a * a,
                        false => a,
                    }
                )
            });
            nums.print(); // [1, 2, 3, 4, 5]
            // if you want nums to change use macro `for_each!` instead
            
13. from_slice

    >**_pub fn from_slice(collection: &[T]) -> ArrayList\<T>_**
    >
    >- Takes a reference to a slice of values and return an ArrayList of the same values.
    
            let fruits = ArrayList::from_slice(&["pineapple", "pear", "banana", "orange"]);
            fruits.print(); //  ["pineapple", "pear", "banana", "orange"]
            
14. get

    >**_pub fn get(&self, index: usize) -> Option\<T>_**
    >
    >- `get` requires the index of the value wanted. It returns the value at the location wrapped in an `option` variant. If the index is greater than the length of the list, the operation panics. Else it returns the value in that index.
    
            let fruits = ArrayList::from_slice(&["pineapple", "pear", "banana", "orange"]);
            // println!("{}", fruits.get(31).unwrap()); // panics
            println!("{}", fruits.get(3).unwrap()); // orange
    
15. index_in

    >**_pub fn index_in(&self, value: usize) -> Option\<T>_**
    >
    > works like `get` method.
    
16. index_of

    >**_pub fn index_of(&self, value: T) -> Option\<usize>_**
    >
    >- Takes a parameter of the value and return the index where the value is located. `index_of` like a reverse of both `get` and `index_in`. If the value is not in the list, the operation panics on unwrap.
    
            //println!("{:?}", fruits.index_of("pine").unwrap()); // panics
            println!("{:?}", fruits.index_of("orange").unwrap()); // 3
            
17. index_of_all

    >**_pub fn index_of_all(&self, value: T) -> Vec\<usize>_**
    >
    >- Takes a value as a paramater, and return `vector` of usize; `Vec<T>`, of all the indexes where the value could be found.
    
            println!(
                "{:?}",
                ArrayList::start_with(
                    &"hello, world is luck to be loud"
                        .chars()
                        .collect::<Vec<_>>(),
                )
                .index_of_all('l')
            );  // [2, 3, 10, 16, 27]
            
18. insert

    >**_pub fn insert(&self, index: usize, value: T)_**
    >
    >- Given two paramaters; the index where the value to be inserted and the value to be inserted. This function, shift, the value in the index (or position) specified and "insert" the new value. Panics if index > length of the list.
    
                let places = arraylist![].add("Lagos").add("Abuja").finish();
                places.print();  // ["Lagos", "Abuja"]
                places.insert(1, "kumasi");
                places.print();  // ["Lagos", "kumasi", "Abuja"]
                
19. is_empty

    >**_pub fn is_empty(&self) -> bool_**
    >
    > Returns true if this list contains no elements, else it returns false.
    
            let al = arraylist!["yah", "yak", "kah"];
            al.clear();
            println!("{}", al.is_empty()); // true
            println!("{}", arraylist![3, 5, 7, 9].is_empty()); // false
            
20. len

    >**_pub fn len(&self) -> usize_**
    >
    >- Returns the length of the list.
    
            let al = arraylist!["yah", "yak", "kah"];
            println!("{}", al.len()); // 3
            al.clear();
            println!("{}", al.len()); // 0
            
21. new

    >**_pub fn new() -> ArrayList\<T>_**
    >
    >- Constructs a new instance of arraylist with default values for the list type.
    
            let al = ArrayList::<String>::new();
            // or
            let al: ArrayList<String> = ArrayList::new();
            
22. pop

    >**_pub fn pop(&self) -> Option<T>_**
    >
    >- Remove and return the last element in the list. Reducing the length and size of the list. The capacity of the list is not affected.
    
                let fruits = arraylist![
                    "orange",
                    "pear",
                    "pineapple",
                    "tangerine",
                    "apple",
                    "strawberry"
                ];
                println!("{}", fruits.pop().unwrap());
                println!("{}", fruits.pop().unwrap());  
                fruits.print(); // ["orange", "pear", "pineapple", "tangerine"]
                
23. print

    >**_pub fn print(&self)_**
    >
    >- Prints out the elements in the list. It feels intutive to call `print` than to use the macro `println!` provideded in rust.
    
                    let fruits = arraylist![
                        "orange",
                        "pear",
                        "pineapple",
                        "tangerine",
                        "apple",
                        "strawberry"
                    ];
                    
                    fruits.print(); //["orange", "pear", "pineapple", "tangerine", "apple", "strawberry"]
                    
24. push

    >**_pub fn push(&self, value: T) -> bool_**
    >
    >- Appends the specified element to the end of this list. But unlike `add`, it does *_NOT_* make the list mutable to do so. Though work like `add`, it can't be used in a "builder" design manner like method `add`. Returns a boolean.
    
                    let al: ArrayList<i32> = ArrayList::new();
                    al.push(-34);
                    al.push(39);
                    al.push(25);
                    
                    al.print(); // [-34, 39, 25]
                    
25. push_on_index

    >**_pub fn push_on_index(&self, index: usize, value: T)_**
    >
    >- Takes two parameters; the index and the value to insert in the list. It works exactly like `insert` method.
    
                    let al = arraylist![1, 2, 3];
                    al.push_on_index(1, 7);
                    al.push_on_index(0, 9);
                    al.print(); // [9, 1, 7, 2, 3]
                    
26. remove

    >**_pub fn remove(&self, index: usize) -> T_**
    >
    >- It removes and return the element of the list specified by the supplied index as parameter. It also reduces the length of the list. Panics if the index > list length.
    
                println!("{}", al.remove(al.len() - 1)); // 3
                al.print();  // [9, 1, 7, 2]
                println!("{}", al.remove(2)); // 7
                al.print();  // [9, 1, 2]
                
27. remove_if

    >**_pub fn remove_if(&self, f: fn(T) -> bool)_**
    >
    >- It works like `remove`, but instead of index, it takes a closure as a parameter and it applies the closure to every element of the list.
    
                let al = arraylist![1, 2, 3, 4, 5, 6, 7, 8, 9];
                    al.print(); // [1, 2, 3, 4, 5, 6, 7, 8, 9]
                    al.remove_if(|a| a % 2 == 0);
                    al.print(); // [1, 3, 5, 7, 9]
                    
28. replace

    >**_pub fn replace(&self, index: usize, value: T)_**
    >
    >- It replaces the elements specified by the index provided, with the new value also specified by the function. It panics if the index is greater the list length.
    
                al.replace(0, 23);
                al.print();  // [23, 3, 5, 7, 9]
    
29. size

    >**_pub fn size(&self) -> usize_**
    >
    >- Returns the number of elements in the list. Works like method `len`.
    
                println!("{}", al.len()); // 5
                
30. start_with

    >**_pub fn start_with(collection: &[T]) -> ArrayList\<T>_**
    >
    >- Construst and return an Arraylist `ArrayList<T>`, using the slice reference given from the parameter. The type of the arraylist is infered from the type of the slice given. The type can also be specified.
    
                let nums = ArrayList::start_with(&[1, 2, 3, 4, 5, 6]);
                    nums.print(); // [1, 2, 3, 4, 5, 6]
    
31. sub_list

    >**_pub fn sub_list(&self, start: usize, stop: usize) -> Option\<ArrayList<T>>_**
    >
    >- Returns from this list all of the elements whose index is between start Index, inclusive, and to stop Index, exclusive. Panic if the to index is greater than the list length.
    
                let nums = ArrayList::start_with(&[1, 2, 3, 4, 5, 6]);
                    nums.print(); // [1, 2, 3, 4, 5, 6]
                    nums.sub_list(1, 4).unwrap().print(); // [2, 3, 4]
                    
32. to_vec

    >**_pub fn to_vec(&self) -> Vec<T>_**
    >
    >- Convert and returns a `vector` with the elements of the list. This is actually great, since rust `vec` has so many mehtod which this wrapper didn't implement. Converting the `arraylist` to vec makes it possible to use these so many methods from vector. 

## TODO

[] Carry out more test and benchmarks for more complex scenarios.
