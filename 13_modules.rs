mod lib;

fn main() {
    lib::fake_file_iterator::read_all_lines("01_hello.rs");
}

// to compile individual module files for use as a library (with no need to recompile at build time)
// $ rustc <module name>.rs --crate-type=lib

// and to link them to the program:
// & rustc <main program>.rs --extern foo=libfoo.rlib

// to include an exernally linked lib in a main:
//   extern crate <module name>;

// rust binaries are large due to the debug information held in them.
// stripping this data is easy:
// $ strip <binary name>

// stripped rust binaries are also quite large because the necessary std lib components are
// stitically linked (built in) to them, making them portable. To reduce further, which involves
// needing the rust runtime on any machine used to run the binary, we can use:
// $ rustc -C prefer-dynamic <main program>.rs --extern <import name>=<lib name>.rlib
// $ strip <main program>
// $ export LD_LIBRARY_PATH=~/.rustup/toolchains/<LINUX|UNIX VERSION>/lib
// $ ldd <main program>
