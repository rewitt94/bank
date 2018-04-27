# Bank

This is a very simple tech-test. The task is to produce an a program that takes deposits and withdrawals with which we can interact with in a REPL (no command line interface tested). Finally, we can print a statement like below.

```
date || credit || debit || balance
14/01/2012 || || 500.00 || 2500.00
13/01/2012 || 2000.00 || || 3000.00
10/01/2012 || 1000.00 || || 1000.00
```
Given an entire week to do tech tests at Makers Academy I decided to spice things up by using hipster language Rust. Here's how it went...

```
Unit tests are in the src code.
```

### Day 1: Foggy

Lots and lots of reading. A day of information. I actually intended to write my tech-tests in Java, but by one in the afternoon I was too excited by Rust to look back. I think what drew me to Rust was familiarity of Cargo (Cargo is a complier/package manager). It made it super easy to initialise new rust projects and was fairly reminiscent of npm. Also I was glad to be able to code in atom opposed to an IDE.

### Day 2: Sunny

By the end of day two I'm feeling confident that I have a grasp of Rust. I have an understanding memory allocation in Rust and how it's garbage collection is working. I spiked a tic-tac-toe app before returning to TDD a Transaction "object" (library projects in Rust are initialised ready for testing). I use an enum, a struct and a corresponding implementation. I finish the day feeling very pleased with myself.

### Day 3: Thunder Storms

First thing I make an Account "object" which will contain a vector (array of unknown length) of Transactions. I want to write and unit test Account and Transaction in isolation. Hence, I should use a dependency injection.

#### [some "light" reading on dependency injection in Rust.](https://pdfs.semanticscholar.org/23a5/8bd6c81d5ff64253647e2acfd5e9cc609d4a.pdf)

Something that would be so trivial in Ruby or JS then becomes the biggest puzzle I've faced at Makers Academy. DI is not simple in Rust. Firstly, Rust is a complied, statically typed language meaning that a vector of Transactions must be a vector Transactions and not a vector of MockTransactions. There is no fooling the complier here. Secondly, there is nothing that could be recognised (by me) as class-based inheritance. So you can't fool the complier by creating a RealTranscation and MockTransaction as a subclasses of the one-true Transaction.

```
Surely something is wrong when you have to make your code 10x more complicated only to test it.
```

In conclusion, no progress was made between 10AM and 6PM. Options for Day 4 are:
 * Find a new way to encapsulate (perhaps I am encapsulating wrong in Rust).
 * Not test in isolation.
 * Do the impossible.
 * Become an Accountant.

### Day 4: Calm

I started to reflect on why I was so desperate to dependency inject my Transactions into my Account. On reflection, it was because how I would TDD in Ruby or in JS. Maybe principles apply slightly differently here.

My Account has no knowledge of the inner workings of my Transaction. It simply calls one of it's two constructor functions and stores it. That behaviour is tested and dependency injection seems to be making a mountain out of a mole hill.

Next I created a writer function (which has some internal private functions). As this function is completely related to formatting Transactions into string literals it makes no sense to test this class in isolation.

I wrote a couple integration tests that match pre-written text files to an output variable (in this case a vector of ASCII codes). The two vectors didn't match perfectly because a new string is created with a line feed (ASCII 10).

In main.rs (the run file) I entered the output variable as stdout to print to the console.

### Library (Crate)

The Publicly available code is:

##### struct Account (with functions):

 * Account::new(starting_balance)
 * account.deposit(amount)
 * account.withdraw(amount)
 * account.history()
 * account.balance()

##### fn output_statement(&account, &buffer)

### Instructions

Clone this repo

```
$ git clone https://github.com/rewitt94/bank.git
```

Install the Rust complier/package manager.
```
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Change into the directory
```
$ cd bank
```

Run the tests
```
$ cargo test
```

Run the main function
```
$ cargo run
```
### Follow-up features

##### Dates

This tech-test is lacking dates. I've looked at implementing the feature but there are the following problems:
 * If I write my transactions to take a date as a parameter I can would pass my tests... but what ATM asks for today's date...?
 * If I make the date to the current time it will break my integration tests as they match to prewritten statements. (Again there is no option of an optional or default parameter in Rust.)
 * More a source of amusement than a problem, but Rust outputs std::time::SystemTime::now() in Unix Time (seconds since 00:00:00 Coordinated Universal Time (UTC), Thursday, 1 January 1970).

### Reflections

Doing this tech-test in a new language was both challenging a fulfilling. Trying to TDD a static complied language made me consider WHY I am writing tests and WHAT behaviour I am testing instead of testing on autopilot.


```
N.B this probably reads too much like a blog post.
```
