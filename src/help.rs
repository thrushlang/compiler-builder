use crate::logging;

pub fn show_help() -> ! {
    logging::write(logging::OutputIn::Stderr, "The Compiler Builder");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "\n\n{} {} {}\n\n",
            "Usage:", "compiler-builder", "[-flag|--flags]"
        ),
    );

    logging::write(logging::OutputIn::Stderr, "Commands:\n\n");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {}, {}, {} {}\n",
            "•", "-h", "--help", "help", "Show help message.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {}, {}, {} {}\n\n",
            "•", "-v", "--version", "version", "Show the version.",
        ),
    );

    logging::write(logging::OutputIn::Stderr, "LLVM build flags:\n\n");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--llvm-major", "Set LLVM major version (default: 17).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--llvm-minor", "Set LLVM minor version (default: 0).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--llvm-patch", "Set LLVM patch version (default: 6).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--llvm-c-compiler", "[clang]", "Set C compiler for LLVM build (default: clang).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-cpp-compiler",
            "[clang++]",
            "Set C++ compiler for LLVM build (default: clang++).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--llvm-c-flags", "[-O3]", "Set C flags for LLVM build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--llvm-cpp-flags", "[-Oz]", "Set C++ flags for LLVM build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-release-type",
            "[Debug|Release|MinSizeRel]",
            "Set LLVM release type (Debug, Release, MinSizeRel) (default: Release).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-build-share-libs",
            "[true|false]",
            "Build LLVM shared libraries (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-build-x86-libs",
            "[true|false]",
            "Build x86 (32-bit) libraries for LLVM (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-build-dylib",
            "[true|false]",
            "Build LLVM dynamic library (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•",
            "--llvm-link-statically-libcpp",
            "[true|false]",
            "Link libcpp statically (default: false).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n\n",
            "•", "--llvm-use-linker", "[lld] Set linker to use for LLVM build.",
        ),
    );

    logging::write(logging::OutputIn::Stderr, "GCC build flags:\n\n");

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "-gcc", "Enable to build GCC backend for the compiler.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--gcc-major", "Set GCC major version (default: 15).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--gcc-minor", "Set GCC minor version (default: 2).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {}\n",
            "•", "--gcc-patch", "Set GCC patch version (default: 0).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--gcc-host-shared", "[true|false]", "Enable host shared for GCC (default: true).",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--gcc-c-compiler-flags", "[-O2 -g]", "Set C compiler flags for GCC build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--gcc-cpp-compiler-flags", "[-O2 -g]", "Set C++ compiler flags for GCC build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n",
            "•", "--gcc-c-compiler-command", "[gcc]", "Set C compiler command for GCC build.",
        ),
    );

    logging::write(
        logging::OutputIn::Stderr,
        &format!(
            "{} {} {} {}\n\n",
            "•", "--gcc-cpp-compiler-command", "[g++]", "Set C++ compiler command for GCC build.",
        ),
    );

    std::process::exit(1);
}
