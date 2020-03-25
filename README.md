snippet
=======

The files in this repo are a brief display of the code I like to write i.e.,
code I'm proud of. Some additional comments were added to `ethereum.rs` to give
context, the other files are all reasonable self explanatory and are in their
original form.

Additionally, here is some context of _why_ I am proud of these snippets.  Apart
for the fact that its just nice clean code.

## Hangman

File: `hangman.py`  
Original: https://github.com/tcharding/hangman/blob/master/hangman.py

### Reason for inclusion

I hacked this up one Sunday afternoon with my kids sitting around the table, the
aim was to spark some interest in programming in them while they were doing
their various other craft activities.

Its included because it is so simple, I feel that code written quickly, simply,
and just for fun is a good representation of ones coding style.

## Leaking addresses

File: `leaking_addresses.pl`  
Original: Any Linux kernel tree under `scripts/leaking_addresses.pl`

### Reason for inclusion

This is a file I authored and maintain within the Linux kernel tree.  I have
included this for a few reasons.  Firstly, its written in Perl.  When I wrote it
I was not at all familiar with the language.  I feel that code written in a
non-familiar language shows ones coding style because it is not tainted by
conforming to the usual style of the language but rather the authors 'goto'
coding style.  Secondly, Perl is often not that readable.  I am proud of this
code because it is [arguably] readable.

## Data

File: `data.go`  
Original: Cannot link to original because it is on my GitHub account in a
private repo at request of the company I currently work for - it was part of the
hiring process I went through i.e., their coding challenge.

### Reason for inclusion

This file is in Golang, a language that I am reasonably familiar with - therefor
no excuses for anything but idiomatic clean code :)

I have included it because it is a fair representation of the style of Golang I
like to write, it lints cleanly, and it is idiomatic Golang (at least to the
best of my knowledge at the time of writing).

## Matching transactions (Ethereum)

File: `ethereum.rs`  
Original: https://github.com/comit-network/comit-rs/blob/dev/cnd/src/btsieve/ethereum.rs

This file is taken from the code base I currently am paid to work on.  It is a
file that I almost completely re-wrote recently.  It is included for a couple of
reasons; firstly it is in Rust, a language I very much enjoy to write. Secondly,
and quite interestingly, the code violates some principles of clean code, namely
there is some duplication (the `Generator` code) and also some of the functions
are quite deeply nested (again, the `Generotor` code).  This code represents the
core functionality of the tool we are writing, since joining the company this
section of code was, in my opinion extremely smelly.  It took concerted effort
on my part to convince the team to allocate time to this part of the code base.
The result, while still complex, is something I am quite proud of.  The original
code is in the git history for the interested.
