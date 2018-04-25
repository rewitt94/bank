# Bank

This is a very simple tech-test. The task is to produce an a program that takes deposits and withdrawals with which we can interact with in a REPL (no command line interface tested). Finally, we can print a statement like below.

```
date || credit || debit || balance
14/01/2012 || || 500.00 || 2500.00
13/01/2012 || 2000.00 || || 3000.00
10/01/2012 || 1000.00 || || 1000.00
```
Given an entire week to do tech tests at Makers Academy I decided to spice things up by using hipster language Rust. Here's how it went...

### Day 1: Foggy

Lots and lots of reading. A day of information. I actually intended to write my tech-tests in Java, but by one in the afternoon I was too excited by Rust to look back. I think what drew me to Rust was familiarity of Cargo (Cargo is a complier/package manager). It made it super easy to initialise new rust projects and was fairly reminiscent of npm. Also I was glad to be able to code in atom opposed to an IDE.

### Day 2: Sunny

By the end of day two I'm feeling confident that I have a grasp of Rust. I have an understanding memory allocation in Rust and how it's garbage collection is working. I spiked a tic-tac-toe app before returning to TDD a Transaction "object" (library projects in Rust are initialised ready for testing). I use an enum, a struct and a corresponding implementation. I finish the day feeling very pleased with myself.

### Day 3: Thunder Storms

First thing I make an Account "object" which will contain a vector (array of unknown length) of Transactions. I want to write and unit test Account and Transaction in isolation. Hence, I should use a dependency injection.

#### [some "light" reading on dependency injection in Rust.](https://pdfs.semanticscholar.org/23a5/8bd6c81d5ff64253647e2acfd5e9cc609d4a.pdf)

Something that would be so trivial in Ruby or JS then becomes the biggest puzzle I've faced at Makers Academy. DI is not simple in Rust. Firstly, Rust is a statically typed language meaning that a vector of Transactions must be a vector Transactions and not a vector of MockTransactions. There is no fooling the complier here. Secondly, there is nothing that could be recognised (by me) as class-based inheritance. So you can't fool the complier by creating a RealTranscation and MockTransaction as a subclasses of the one-true Transaction.

```
Surely something is wrong when you have to make your code 10x more complicated only to test it.
```

In conclusion, no progress was made between 10AM and 6PM. Options for Day 4 are:
 * Find a new way to encapsulate (perhaps I am encapsulating wrong in Rust).
 * Not test my classes in isolation.
 * Do the impossible.
 * Become an Accountant.

### Day 4: blank

### Day 5: blank

```
N.B this probably reads too much like a blog post.
```
