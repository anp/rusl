# rusl

This is a super experimental attempt at re-implementing [musl](http://musl-libc.org) in Rust. No progress to report yet.

## A little background

[musl](http://www.musl-libc.org/) is, according to its homepage, "a new standard library to power a new generation of Linux-based devices. **musl** is *lightweight, fast, simple, free,* and strives to be *correct* in the sense of standards-conformance and safety."

[Rust](https://www.rust-lang.org) is, according to its homepage, "a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety."

Seems like a nice match! In fact, there's been some great work done to [make Rust executables statically compile with musl as the libc implementation on Linux](http://blog.rust-lang.org/2016/05/13/rustup.html), allowing for fully static Rust Linux binaries (a la Go). musl has also [been cited as an excellent codebase](http://blog.regehr.org/archives/1393) for someone to read when learning C. But wait, if musl's C code is already so great, why rewrite it in Rust?

1. Because I don't know C very well, and I wanted to tinker with a codebase mostly free from preprocessor magic and with sane design and formatting decisions. As an added bonus, musl's source is very well chunked into smaller files with few interdependencies, which means that an incremental porting approach can work well.
2. musl has a [pretty big test suite](http://wiki.musl-libc.org/wiki/Libc-Test). As a systems programming (whatever that means) novice, I don't want to trust in my non-existent psychic powers to detect regressions that result from my actions.
3. I thought it'd be fun to have C programs call `malloc()` and execute Rust (and right now, the test suite currently does and passes!). It could also (if I get a heck of a lot further than I have) be a cool test case for demonstrating Rust's potential to improve our current computing infrastructure and ecosystem.

## Goals and non-goals

It's obviously stupid for one junior programmer to try to build their own C standard library, right? Yeah, it is. But there are still a few reasons I'm liking this:

* Learning: I don't know C and its domains very well. In my day job I write back-end web code, data analysis scripts, and bioinformatics tools. Mostly in Python. This seems like a great way to learn about C, how software interacts with the kernel, and what goes into making a standard library.
* Showing that Rust can do this kind of work (allocators, pthread implementations, etc.), and showing myself that I'm capable of learning how to :).
* Experimenting with ways to incrementally consume a C library or program from the inside out using Rust's solid C interop support.
* I'm starting to get a more realistic idea of Rust's current limitations when operating with C (I'm looking at you, unstable inline assembly and unimplemented untagged union support).

There are also a whole bunch of things I have no intention of doing:

* Using [rust-bindgen](https://github.com/crabtw/rust-bindgen) or some other automated tool to generate type declarations or translate code for me. As a learning project, it's much better if I hand-translate everything (which so far is just a tiny portion of the overall codebase).
* Ever having this code used in production. Ever. No really, ever. The test suite that I'm using has gaps (if you're a POSIX aficionado and a musl fan, I'm sure they'd love help improving the coverage), and while I feel pretty confident about higher-level work I do, I'm dumb as a rock when it comes to pointer math.
* Supporting any platform other than Linux on x86_64. Or really supporting any platform other than my laptop or desktop.
* Trying to improve upon what musl actually does. If a bug gets fixed accidentally, then cool. But right now I'm just trying to replicate its behavior nearly exactly in Rust.
