# Tic Tac Toe - Written in pure std-[Rust](https://www.rust-lang.org/)

It uses some slightly advanced language features.  
For example implementing `trait`s like `Display` or `Deref`.  
Also used is the `impl Trait` syntax in some function arguments or as returntype from a function.  
This is mainly done for training purposes in this play-program.  
But the reason by this is to reduce heap allacations, which would happen if the the types were collected before the are passed around.
