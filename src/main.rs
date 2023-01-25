#![allow(unused)] // to ignore unused warnings to make the error log easier to read

// This project is made for learning how to crate a library aka crate and export to crates.io (in addition to practice sorting algos)
// The goal of this project is to undersand how to build a simple function using external crate, modules and any C library (.c/.dll/.so)
// and use the library from other project by injecting the dependency.
// 
// This crate is only used for learning purpose from building a library to writing docs, so please DON'T use this crate for own project.
// There are a few simple sorting algos for Vec of i32 in this package and there are some testing and branchmark script
// I will import a few c / dll / and so files to test how to add c code into the library
use crate_creation_to_export_and_beyond::bubble_sort::bubble_sort;

fn main() {
    println!("The library in action");

    let input = vec![257,47,27,89,312,9,654,3,45,87,456,234,34,67,9,77,54,1,16,67,87,98,723,65];
    let output = bubble_sort(&input);

    println!("{:?}", input);
    println!("{:?}", output);
    
}
