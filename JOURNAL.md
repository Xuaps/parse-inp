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

