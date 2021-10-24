# Tic Tac Toe - Written in pure std-[Rust](https://www.rust-lang.org/)

It uses some slightly advanced language features.  
For example implementing `trait`s like `Display` or `Deref`.  
Also used, is the `impl Trait` syntax (especially `impl Iterator<Item = ..>`) in some function arguments or as a return type from a function.  
This is mainly done for training purposes in this play-program.  
But the reason behind it is to generalize the api and to reduce heap allacations, which would happen if the types were first collected (i.e. into a `Vec`) before the are passed around.
