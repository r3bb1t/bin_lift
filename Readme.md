# bin_lift


Inspired by [RetDec](https://github.com/avast/retdec) and it's [capstone2llvmir](
https://github.com/avast/retdec/tree/master/src/capstone2llvmir) module as well as 
[remill](https://github.com/lifting-bits/remill) (and later [Mergen](https://github.com/NaC-L/Mergen)) this project aims to serve as a convenient library used for serving same
purpose. It allows you to convert raw binary data to LLVM IR (Intermediate representation).


# Dependencies

Tested on Ubuntu 24.

Core IR generation is pure Rust plus Zydis and the sibling `llvmkit` crate.
LLVM command-line tools are only needed for optional external post-processing of the emitted `.ll`, not for lifting or verification.
