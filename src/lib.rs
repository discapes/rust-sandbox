// lib.rs: this is a library crate
pub mod mymod; // found in src/submod.rs or src/submod/mod.rs
               // use pub to make this module public with this crate
               // basicall each file in rust requires a mod in another file

mod privateandinline {
    pub mod baz {
        pub fn asd() {}
    }
}

use mymod::bar;

pub fn foo() -> i32 {
    privateandinline::baz::asd();
    return 15 + bar();
}
