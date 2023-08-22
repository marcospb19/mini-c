# mini-c

My (_initial_) experiments building compilers with LLVM.

# How

I'm generating LLVM-IR using `inkwell`, it's the only decent library I found.

After generating the IR, there are two options:

1. Running in LLVM's JIT execution engine (commented out in the code).
2. Dumping the IR into a file and using several utils to create the final binary.

I'm doing the latter, which is cumbersome but let's me inspect the generated assembly.

Here's the huge command I'm running:

```sh
clear \
    && cargo run -- file.c \
    && llc llvm-ir.ll -o assembly.s \
    && clang assembly.s -o a.out \
    && bat file.c \
    && (./a.out ; echo "\$? = $?")
```

Here's the oneliner:

```sh
clear && cargo run -- file.c && llc llvm-ir.ll -o assembly.s && clang assembly.s -o a.out && bat file.c && (./a.out ; echo "\$? = $?")
```

1. My program compiles file.c to LLVM IR and dumps it to the file `llvm-ir.ll`.
2. `llc` compiles `llvm-ir.ll` into the assembly file `assembly.s`.
3. `clang` compiles `assembly.s` into machine-code, `a.out`.
4. Show the source code, run the program (output appears here) and then print the return code.

You need `llvm` installed for `lcc` and `clang`.

You can also use `gcc` instead of `clang` here.

# Dependencies for building

If you try running `cargo run` you'll meet a unpleasant linking error.

In some systems, installing `llvm` may solve, in Arch Linux you need to download a LLVM release tarball and unpack it.

https://github.com/llvm/llvm-project/releases/tag/llvmorg-15.0.0

Then point an env var to the resulting folder like so:

```sh
export LLVM_SYS_150_PREFIX=$(realpath ../llvm/clang+llvm-15.0.0-x86_64-linux-gnu-rhel-8.4)
```

Now `llvm-sys` and `inkwell` will compile fine, make sure you open your text editor of choice after setting this env var.
