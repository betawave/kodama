# Kodama

### What's a Kodama?

A Kodama - originally the name of japanese tree spirits - is a program for interfacing with some kind of tree structure.  Specifically, a [LANG]-Kodama may be used to work with a [LANG] syntax tree representing a program.  It may be used to create, destroy, modify and show a particular syntax tree.

### What is the purpose and plan of this *project*?

Via the Kodama project we want to explore the ideas behind structure editors and how they may be combined with other code editing ideas like Vim's modal editing, Emacs' unlimeted extensibilty, etc.

Project Agenda:  
1. Implement a very basic tree Kodama and CLI editor frontend to evaluate most abstract editing experience.
   For further reasoning of this step see [TODO](). After evaluation either:
    1. stop project if not promising
    2. continue with 2. if interesting
2. Implement a Rust-Kodama and integrate it into Emacs
3. Reimplement the Rust-Kodama using the initial Rust-Kodama (i.e. bootstrap).
   This should provide us with real world experience into the usability of the concept.

### What is the purpose and structure of this *repository*?

The purpose of this repository is threefold:  

First of, in [src](./src), it contains the code that we use for exploring the abtract/minimal tree Kodama, the initial Rust-Kodama and finally the bootstrapped Rust-Kodama.

Secondly, in [TODO](), it contains notes where we describe the concepts we find and explore. This is also where our evaluations and experiences will be documented.

Lastly, in [TODO](), it contains standards documents which describe the interfaces a program should implement to call itself a Kodama.
