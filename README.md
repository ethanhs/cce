# cce - command line compiler explorer

[![Travis](https://img.shields.io/travis/ethanhs/cce.svg?style=flat-square)](https://travis-ci.org/ethanhs/cce) | [![Appveyor](https://img.shields.io/appveyor/ci/ethanhs/cce.svg?style=flat-square)](https://ci.appveyor.com/project/ethanhs/cce)

Do you love [Compiler Explorer](https://godbolt.org/)? Do you like the command line? Well this is the tool for you!

[![asciicast](https://asciinema.org/a/lRhuNuTp33d8i2aMwhdNVd8ON.svg)](https://asciinema.org/a/lRhuNuTp33d8i2aMwhdNVd8ON)

## Building cce

With a valid rust toolchain (I like [rustup.rs](https://rustup.rs/)) you can just
```
cargo build --release
```

And the binary should be located at `./target/release/cce`

## Using cce

cce has 3 main operations:

### Listing languages

```
~> cce list langs
c++
cppx
assembly
cuda
llvm
d
ispc
analysis
c
rust
go
pascal
haskell
swift
```

### Listing compilers

```
~> cce list compilers
Name: ARM MSVC 2017 RTW, Id: cl19_arm
Name: ARM gcc 4.5.4 (linux), Id: armg454
Name: ARM gcc 4.6.4 (linux), Id: armg464
Name: ARM gcc 5.4 (linux), Id: armhfg54
Name: ARM gcc 5.4.1 (none), Id: arm541
Name: ARM gcc 6.3.0 (linux), Id: armg630
Name: ARM gcc 7.2.1 (none), Id: arm710
Name: ARM64 gcc 5.4 (linux), Id: aarchg54
Name: ARM64 gcc 6.3.0 (linux), Id: arm64g630
Name: AVR gcc 4.5.4, Id: avrg454
Name: AVR gcc 4.6.4, Id: avrg464
Name: Latest trunk, Id: cppx_trunk
Name: MIPS gcc 5.4, Id: mips5
Name: MIPS gcc 5.4 (el), Id: mips5el
Name: MIPS64 gcc 5.4, Id: mips564
...
```

You can also pass a language as a filter to listing compilers:

```
~> cce list compilers -l rust
Name: rustc 1.0.0, Id: r100
Name: rustc 1.1.0, Id: r110
Name: rustc 1.10.0, Id: r1100
Name: rustc 1.11.0, Id: r1110
Name: rustc 1.12.0, Id: r1120
Name: rustc 1.13.0, Id: r1130
Name: rustc 1.14.0, Id: r1140
Name: rustc 1.15.1, Id: r1151
Name: rustc 1.16.0, Id: r1160
Name: rustc 1.17.0, Id: r1170
Name: rustc 1.18.0, Id: r1180
Name: rustc 1.19.0, Id: r1190
Name: rustc 1.2.0, Id: r120
Name: rustc 1.20.0, Id: r1200
Name: rustc 1.21.0, Id: r1210
Name: rustc 1.22.0, Id: r1220
Name: rustc 1.23.0, Id: r1230
Name: rustc 1.24.0, Id: r1240
Name: rustc 1.25.0, Id: r1250
Name: rustc 1.26.0, Id: r1260
Name: rustc 1.3.0, Id: r130
Name: rustc 1.4.0, Id: r140
Name: rustc 1.5.0, Id: r150
Name: rustc 1.6.0, Id: r160
Name: rustc 1.7.0, Id: r170
Name: rustc 1.8.0, Id: r180
Name: rustc 1.9.0, Id: r190
Name: rustc beta, Id: beta
Name: rustc nightly, Id: nightly
```

### Compiling!

Finally, once you know the compiler id you would like to use, you can compile, passing arguments after `--`:

```
~> cce compile arm64g630 -- -O3
<opens an editor set via $VISUAL or $EDITOR>
Compiling with arm64g630 compiler outputs:

main:
        mov     w0, 1
        ret
```

You can also pass a file after the compiler ID:

```
~> cce compile g81 test.c
Compiling with g81 compiler outputs:

main:
        mov     eax, 1
        ret
```

And you can also get an URL for the compilation job:
```
~> cce compile --url g81 test.c
URL: https://godbolt.org/#%7B%22content%22%3A%5B%7B%22content%22%3A%5B%7B%22componentName%22%3A%22codeEditor%22%2C%22componentState%22%3A%7B%22id%22%3A1%2C%22options%22%3A%7B%22colouriseAsm%22%3Atrue%2C%22compileOnChange%22%3Atrue%7D%2C%22source%22%3A%22int%20main%28%29%20%7B%5Cr%5Cn%5Ctreturn%201%3B%5Cr%5Cn%7D%5Cr%5Cn%22%7D%2C%22type%22%3A%22component%22%7D%2C%7B%22componentName%22%3A%22compiler%22%2C%22componentState%22%3A%7B%22compiler%22%3A%22g81%22%2C%22filters%22%3A%7B%22commentOnly%22%3Atrue%2C%22directives%22%3Atrue%2C%22intel%22%3Atrue%2C%22labels%22%3Atrue%2C%22trim%22%3Atrue%7D%2C%22options%22%3A%22%22%2C%22source%22%3A1%7D%2C%22type%22%3A%22component%22%7D%5D%2C%22type%22%3A%22row%22%7D%5D%2C%22version%22%3A4%7D
```

The full help listing:

```
cce - a command line interface to compiler explorer 0.1.0
Ethan Smith
Input C++, C, Rust, Haskell, Swift, etc, get assembly
USAGE:
    cce [OPTIONS] <SUBCOMMAND>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
        --host <host>     specify the Compiler Explorer host [default: https://godbolt.org]
SUBCOMMANDS:
    compile    Compile a snippet on compiler explorer
    help       Prints this message or the help of the given subcommand(s)
    list       List the compilers and languages available on compiler explorer
```

### License

This project is under the MIT license. By contributing, you agree to license your
work under the MIT license. See LICENSE for more.
