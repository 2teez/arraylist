
# arraylist

## Name

`arraylist` -- An intutive rust vector wrapper built on top of rust vector implementation. Taste like Java Arraylist, and can be used like Python list functons and JavaScript array.

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
With arraylist, it works:
```
let arr = arraylist![1, 2, 3, 4];
arr.push(5);
arr.push(6);

// print out your arraylist like so:
arr.print(); // [1, 2, 3, 4, 5, 6]
```
And yet your `arr` is still immutable.
