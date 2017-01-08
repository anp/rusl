# rusl

[![Build Status](https://img.shields.io/travis/dikaiosune/rusl/master.svg?style=flat-square)](https://travis-ci.org/dikaiosune/rusl) [![License](https://img.shields.io/badge/license-MIT-lightgray.svg?style=flat-square)](https://github.com/dikaiosune/rusl/blob/master/LICENSE)

This is a super experimental attempt at re-implementing [musl](http://musl-libc.org) in Rust.

## Blog post(s)

* [Baby Steps: Slowly Porting musl to Rust](http://blog.adamperry.me/rust/2016/06/11/baby-steps-porting-musl-to-rust/)

## Running tests

If you'd like to run the tests (right now rusl only support x86_64 Linux), clone the repo and run:

```
scripts/clean_all.sh && scripts/build_and_test.sh
```

## Contributing

If you want to contribute -- thanks, I'm flattered! Please be aware that this
just a hobby project, so reviews may not be very rapid. Also, while CI is
configured, please please please make sure your code builds and tests pass using
a recent nightly on an x86_64 Linux box/VM/VPS. It will save a lot of time
before the submission gets to the review phase.

CI is now set up for Pull Requests, so that needs to pass before a PR is
reviewed. There aren't enough contributions right now to write a style guide,
but before submitting anything please read some of the existing code to get a
feeling for the current style before submitting anything, and also run
[rustfmt](https://github.com/rust-lang-nursery/rustfmt) on your submission using
this repository's configuration. Since this is just my hobby project at the
moment, please be patient if I nitpick your PR -- there are no practical goals
here so I'd like to keep things nice (even if it's at the expense of time spent
or contributions lost).

Please see issue #6 if you'd like help figuring out a starting point for
contributions.

### Tips & Caveats

As you port musl functions to Rust, you'll eventually expose one or more `pub`
functions in `rusl`. For example, `malloc` from `musl/src/malloc/malloc.c` is
ported to `malloc` in `src/malloc/malloc.rs`. Other (non-public, `static`)
functions from `malloc.c` might get ported but won't necessarily be exposed as
`pub`lic. With new `pub` functions `librusl.a` will contain symbols which
conflict with object files in musl's `libc.a`, and the build will fail. The
solution is to put the name of the object file with the conflicting symbols in a
file called `ported_objects` living in the root of the repo. It describes all
the object files which should be deleted from musl's `libc.a` so we don't get
symbol conflicts during linking. One small caveat is that a few functions
(`memset`, `memcmp`, and friends) are provided by `rlibc`, not `rusl`, but we
still need to remove the symbols from musl's `libc.a` before mixing in `rusl`
symbols.

Effectively, this means you should not expose new `pub`lic functions in `rusl`
until an entire C source file has been ported. Don't worry; let build failures
be your guide.

You can use `nm bld/usr/lib/libc.a | less` to verify the object filename
providing a given symbol. Sometimes they don't match (i.e., `vmlock.c` -->
`__vm_lock`).

## Documentation

Since musl doesn't include documentation itself, this project won't make any effort to add new documentation beyond any comments necessary to clarify implementation-specific behavior. For documentation of C language functions and the POSIX C standard library, see:

* [C99 language standard](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
* [POSIX System Interface Functions](http://pubs.opengroup.org/onlinepubs/9699919799/functions/contents.html)

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

## LICENSE

Like musl, rusl is made available under the MIT license.
