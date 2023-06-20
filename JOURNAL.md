## 2023 06 19

### properties and comment, make read functions pure and create a sectionable trait

This time I had clear that I wanted to remove the code repeated code on the read_x methods. I started extracted a funtion that given a line returns a vector of strings, the properties and a comment. I continued returning the section struct from this methos and leaving the main one mute the inp struct. Finally I though I could encapsulate the logic to build every struct in a from method implementation.

I think the code is now quite clear as the strategy for parsing every section. To add a new section I only need to add the struct and the implementation of the from method.

There is an extra thing pending, if the algorithm find one section and it is not implemented it will panic. I have though on include a uknown struct to accumalate all the section not defined.

## 2023 06 07

### Multiline and comments

I have changed my mind again. I have created the line abtraction and instead of travel char by char now I process entire lines. This change has simplified the implementation since I don't have to take care of when a new line start. Still I don't want to delegate the handling of line cllection to method that processes lines because I don't want to mutate the iterator in a differnet context. I keep the current section in a variable and I continue processing every line as part of that section until it changes and I am ready to start with a new one. 

I still have a lot of doubts about Rust, when to use string or str, best approached to do something. I am still trusting on copilot to do this part of the job, however it is not as consistant as it should be creating code.

## 2023 05 30

### Use two different collections

I am thinking on borrow some idea from other project and use a cloned collection to travel the sections and another one to go from one section to another. I don't like to mute the chars collections inside fuction call readX. A part of that,  have implemented a very basic version of the readReservoirs, but it is enought to see a clear approach. i still have to deal with all the things that can go wrong: bad format, comments, missing values, etc. Lets move to this new approach and lets see how can I accomodate all this problems in the final design.


## 2023 05 20

### Is it a good idea to use a parser for this?

As I am not needing an AST, probably doesn't make any sense to invest in tokenize the whole text. It is easier to create a data structure that contains the data in the file in a way that we can consume it easily.

At the end I have changed the strategy, and I have implemented a INP struct that has a read function to fill it with the input string. Let see. 

I am getting use to work with copilot and I have to say that it feels weird but, at the end, it is quite productive.

