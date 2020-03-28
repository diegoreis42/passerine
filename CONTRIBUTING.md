# Contributing to P
If you're reading this, thanks for taking a look!
If you would like to contribute to P, we have some simple rules to follow.
Contributions are made through pull requests, unless you are given write access to the repository.\*

## General Guidelines

First, if you find any bugs or would like to request any features, please open an issue.
Once the issue is open, feel free to put in a pull request to fix the issue.
Small corrections, like fixing spelling errors, or updating broken urls are appreciated.
(Note that feature requests, even if implemented, might not be accepted if they don't align with the goals of the project.)

Second, all pull requests must have adequate testing and peer-review to assure proper function before being accepted.
If your code does not compile, or introduces other obvious bugs, your pull request will not be accepted.
More likely than not, it will take more that one commit to get a pull request in.
Don't get discouraged if your fist solution doesn't solve the problem, try again!

Third, we encourage an open, friendly, and supportive environment around the development of P.
If you disagree with someone for any reason, discuss the issue and express you opinions, don't attack the person.
Discrimination of any kind against any person is not permitted.
If you detract from this project's collaborative environment, you'll be prevented from participating in the future development of this project until you prove you can behave yourself adequately.
Please provide arguments based on anecdotes and reasoning to support your suggestions - don't rely on arguments based on 'years of experience,' supposed skill, job title, etc. your points across.

## P-Specific Guidelines for Getting Started
> Note: this project is in the rapid initial stages of development, and as such, breaking changes or radical changes in project structure may occur. After the 1.0.0 release, this behavior will stabilize.

> TODO: integrate idea that readable code that works is better than unreadable code that's only a bit faster

P strives to implement a modern compiler pipeline. P is currently broken up into three small projects, namely, the core compiler, the command line interface, and the package repository.

### Core Compiler
The core compiler is made of a series of 'pipes,' which transform one type of data into another.
Programs start as source files, after which they are then lexed into tokens, parsed into ASTs, compiled to bytecode, then run on the VM.

The pipes themselves (i.e., the lexer, the parser, and the bytecode generator), can be found in `src/pipes`.
The datastructures associated with these pipes can be found in `src/pipeline`.

Note that pipes can only reference datastructures in pipeline - the pipeline should never reference anything found in pipes.
Additionally, pipes should *not* rely on other pipes to get their job done.
We follow the philosophy of Unix\*\*: Each pipe should do one thing, and do it well.

Pipes can, however, rely on datastructures in `src/utils` and `src/vm`.
For instance, all datastructures are annotated with `Annotations`, which point to which part of the source file each each structure represents.

`src/utils` contains utilities that can be used throughout the entire compiler.
This includes utilities for printing errors, building the P standard library, annotating source files, and passing data through multiple pipes sequentially.

The powerhouse of this operation, the VM and its associated datastructures, can be found in `src/VM`. The VM is expected to only rely on bytecode and a constants table. Pipes should not to rely on the VM, unless they are producing a VM-specific datastructure.

As for testing, each non-trivial file is expected to have unit tests at the bottom of it.
As a rule of thumb, at least a third of a file should be dedicated to testing.

The easiest way to contribute is to find errors and report them.
If you run into an error, it would be appreciated if you could go the extra mile and write some test cases that show the error.

### CLI
The VM should be manipulated solely through the utilities provided by core compiler.

> NOTE: this phase of the project has not started yet.

### Package Repository
P supports modules (local, un-versioned packages) and packages. All packages and their versions to be used by a P project are defined in a `packages.icn` (Isaac's Configuration Notation) file in the project's root.

> NOTE: this phase of the project has not started yet.

```
# example .icn
# each key maps to a list of keys, or a list of values

metadata:
    # list of keys
    name: Example  # list of length 1
    authors:
        # list of values
        Isaac Clayton
        Someone Else
    version: 1.2.1

dependencies:
    http: 0.63.2
    graph: 2.9.5
```

## Footnotes
> \* This is a great responsibility. Even if you have write access, it is suggested you always open a pull request. If you're patching an urgent non-breaking-change, pushing to master is acceptable.

> \*\* However, we do not believe everything is a file.