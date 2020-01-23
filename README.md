# ruesti-base
Base library for Rüstigraben

## What?

This is a library-in-progress to run Rust code on [GraalVM](https://www.graalvm.org/).

## Why?

Running native code on the JVM is a pain, because you'll have to distribute native libraries for each OS along your classfiles.
GraalVM comes with an interpreter for LLVM bitcode which is OS-independent.
GraalVM Community Edition cannot run LLVM in sandboxed mode, so we need to use a memory-safe language like Rust.

## How?

We compile Rust code in a pretty standard way, except we need configure the crate to be a `staticlib` so that all dependencies end up in the bitcode that is emitted by the compiler.

If you want to try it out, here is how:

```
sbt test
```

This should run Cargo and then some tests which will hopefully print a hello world.
You can get sbt [here](https://git.io/sbt).
