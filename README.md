# Kodama

> We hold these truth to be [argument-necessitating], that code is not text and should not be treated as such[^1], that code in textual representation has advantages which need to be retained[^2], that we should not parse but serialize[^3], that code - as well as thought - should be freed from the shakles of sequential narratives implied by text files and pages[^4], that syntax is bliss[^5], that metaprogramming should be a right of all programming languages and not a feature of a few[^6], that in the context of current human-machine interface in use during programming one should rarely leave the home row - i.e. typing is preferable to pointing and clicking[^7], that intuition builds on abstraction[^8], that learnabilty of a user interface requires immediate observability of cause and effect with intermediate steps in case of complex, composite actions[^9], that usability is strongly linked to familiarity and is therefore a conservative metric[^10], that the syntax system should aid in the development of corrent programs and not merely reject faulty ones[^11], that language tooling should be able to process partially completed programs - i.e. programs with holes - and expose language information to help fill these gaps[^12]  [...]

On a serious note, all that we are saying in this repository is not based on empirical data or extensive research.  We are only some dude(tt)s with personal experiences, ideas and prefrences.

[^1]: See TODO
[^2]: See TODO
[^3]: See TODO
[^4]: See TODO
[^5]: See TODO
[^6]: See TODO
[^7]: See TODO
[^8]: See TODO
[^9]: See TODO
[^10]: See TODO
[^11]: See TODO ~ Mention Idris
[^12]: See TODO ~ Maybe same as above

## NOTE: Try Emacs' [`evil-tree-edit`](https://github.com/ethan-leba/tree-edit) for hands-on experience of structure x modal editing!

If you have a python project under active developmend at hand and you want to experience how structure editing compbined with modal editing might feel, have a go with [`evil-tree-edit`](https://github.com/ethan-leba/tree-edit)!  
It is by far not complete or polished but is easy to install and its feature set is advanced enough to get a good impression.  
In this project we aim for a similar interface, but our idea goes beyond `tree-edit` as we suggest that text might not be the best representation for code which remains unchallenged by projects like `tree-edit`, etc.

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

## Related Work

### Software - Projects, Products & Extensions

- [Project - Sapling: Very Similar Project - Vi like Structure Editor written in Rust](https://github.com/kneasle/sapling)
   - only works with JSON, can not load or save
- [Maybe;To be checked] [Tree Sitter](https://tree-sitter.github.io/tree-sitter/)
   - [Emacs Extension - tree-edit](https://github.com/ethan-leba/tree-edit)
      - interactive structure editing, usable but rough and below usefulness vs. pain threshhold
   - [To be checked] [nVim Plugin - Architext: Structure Editing in Vim](https://github.com/vigoux/architext.nvim)
- [(dead) Project - Fructure: Very Nice Structure Editing Prototype with many good ideas!](https://github.com/disconcision/fructure)
   - seems to be dead
   - Best UI so far
      - real eye-candy
      - interaction design is superb and should be an inspiration
   - subset of scheme only
- [(dead?) Project - Programtree](http://www.programtree.com/intro.htm) 
- [Project - Forest: Structure Editor for TypeScript](https://github.com/tehwalris/forest)
- [Emacs Extension - GopCaml Structure Editor for OCaml](https://github.com/Gopiandcode/gopcaml-mode)
- [Maybe;To be checked] [Product - Treefrog](https://treefrog-editor.com/intro/)

### Ideas - Papers, Blogposts, Forums & Talks

- [Paper - Programming environments based on structured editors : the Mentor experience](https://www.researchgate.net/publication/29650883_Programming_environments_based_on_structured_editors_the_Mentor_experience)
- [Paper - Structure Editors: Old Hat or Future Vision?](https://link.springer.com/chapter/10.1007/978-3-642-32341-6_6), [free download](https://projects.fbi.h-da.de/~b.humm/pub/Gomolka_Humm_-_Structure_Editors__Springer_ENASE_.pdf)
- [Msc. Thesis - Forest: Structure Editor for TypeScript](https://www.research-collection.ethz.ch/handle/20.500.11850/526812), [pdf](https://www.research-collection.ethz.ch/bitstream/handle/20.500.11850/526812/masters-thesis-final-eth-Voinov-title-page.pdf?sequence=1&isAllowed=y), [talk on youtube](https://www.youtube.com/watch?v=ze_nJlKkckg)
- [Paper by Huet - The Zipper](https://dl.acm.org/doi/10.1017/S0956796897002864), [pdf](https://www.st.cs.uni-saarland.de/edu/seminare/2005/advanced-fp/docs/huet-zipper.pdf)
   - [Paper - Clase: cursor library for a structured editor](https://dl.acm.org/doi/10.1145/1543134.1411302), [pdf](http://www.doc.ic.ac.uk/~tora/clase/CLASE-Short.pdf)
   - [Paper - GopCaml Structure Editor for OCaml](https://arxiv.org/abs/2207.07423), [tutorial](https://discuss.ocaml.org/t/introducing-gopcaml-mode-structural-ocaml-editing/5310)
- [TODO] [Paper - Robust Projectional Editing](https://voelter.de/data/pub/robustProjectionalEditing.pdf)
- [List - List of Projectional Editors](https://www.alexeyshmalko.com/20200830010958/)
- [List - Awesome Structure Editors](https://github.com/yairchu/awesome-structure-editors/blob/main/README.md)
- [Blogpost - Martin Fowler: ProjectionalEditing](https://martinfowler.com/bliki/ProjectionalEditing.html)
- [Subreddit - Projectional Programming](https://www.reddit.com/r/nosyntax/)
- [Maybe;To be checked] [Language Server Protocol](https://microsoft.github.io/language-server-protocol/)
- [Maybe;To be checked] [Proposed Msc. Project @ Uni Tübingen: Refactoring Macros](http://ps.informatik.uni-tuebingen.de/teaching/thesis/2019/11/25/refactoring-macros/)

## Tangentially Related

- [Paper - The Semantics of Syntax (PDF)](https://conf.researchr.org/getImage/OBT-2016/orig/OBT_2016_paper_10.pdf)
- [Vis - Modal Editing x Structural Regular Expressions](https://github.com/martanne/vis)
