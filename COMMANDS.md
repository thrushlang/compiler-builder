<img src= "https://github.com/thrushlang/.github/blob/main/assets/logos/thrushlang-logo.png" alt= "logo" style= "width: 80%; height: 80%;"></img>

# Compiler Builder | Commands & Flags

A list of the commands supported by the compiler builder command line.

> [!WARNING]  
> This might be a bit outdated, it could be information that's somewhat distant from the changes.

```console
The Compiler Builder

Usage: compiler-builder [-flag|--flags]

Commands:

• -h, --help, help Show help message.
• -v, --version, version Show the version.

LLVM build flags:

• --llvm-major Set LLVM major version (default: 17).
• --llvm-minor Set LLVM minor version (default: 0).
• --llvm-patch Set LLVM patch version (default: 6).
• --llvm-c-compiler [clang] Set C compiler for LLVM build (default: clang).
• --llvm-cpp-compiler [clang++] Set C++ compiler for LLVM build (default: clang++).
• --llvm-c-flags [-O3] Set C flags for LLVM build.
• --llvm-cpp-flags [-Oz] Set C++ flags for LLVM build.
• --llvm-release-type [Debug|Release|MinSizeRel] Set LLVM release type (Debug, Release, MinSizeRel) (default: Release).
• --llvm-build-share-libs [true|false] Build LLVM shared libraries (default: false).
• --llvm-build-x86-libs [true|false] Build x86 (32-bit) libraries for LLVM (default: false).
• --llvm-build-dylib [true|false] Build LLVM dynamic library (default: false).
• --llvm-link-statically-libcpp [true|false] Link libcpp statically (default: false).
• --llvm-use-linker [lld] Set linker to use for LLVM build.

GCC build flags:

• -gcc Enable to build GCC backend for the compiler.
• --gcc-major Set GCC major version (default: 15).
• --gcc-minor Set GCC minor version (default: 2).
• --gcc-patch Set GCC patch version (default: 0).
• --gcc-host-shared [true|false] Enable host shared for GCC (default: true).
• --gcc-c-compiler-flags [-O2 -g] Set C compiler flags for GCC build.
• --gcc-cpp-compiler-flags [-O2 -g] Set C++ compiler flags for GCC build.
• --gcc-c-compiler-command [gcc] Set C compiler command for GCC build.
• --gcc-cpp-compiler-command [g++] Set C++ compiler command for GCC build.
```