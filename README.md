# x86\_64-unknown-uefi rust dwarf symbols reproducer

## Nightly version:
```
nightly-x86_64-unknown-linux-gnu
rustc 1.78.0-nightly (d18480b84 2024-03-04)
```
## Generation of custom target
```bash
rustc +nightly -Z unstable-options --target=x86_64-unknown-uefi --print target-spec-json > x86_64-unknown-dwarf-uefi.json
```

## Building 
```bash
cargo +nightly build --target x86_64-unknown-dwarf-uefi.json -Zbuild-std=core
```

## GDB output


### No variables present
```bash
$ gdb -ex "info variables" -ex "quit" target/x86_64-unknown-dwarf-uefi/debug/uefi_dwarf_example.efi
GNU gdb (Ubuntu 14.0.50.20230907-0ubuntu1) 14.0.50.20230907-git
...
Reading symbols from target/x86_64-unknown-dwarf-uefi/debug/uefi_dwarf_example.efi...
All defined variables:

File ~/.cargo/registry/src/index.crates.io-6f17d22bba15001f/compiler_builtins-0.1.108/src/x86_64.rs:
90:     i32;

```

### Function present
```bash
$ gdb -ex "info functions" -ex "quit" target/x86_64-unknown-dwarf-uefi/debug/uefi_dwarf_example.efi
GNU gdb (Ubuntu 14.0.50.20230907-0ubuntu1) 14.0.50.20230907-git
...
Reading symbols from target/x86_64-unknown-dwarf-uefi/debug/uefi_dwarf_example.efi...
All defined functions:

File src/main.rs:
10:     fn uefi_dwarf_example::main(*mut core::ffi::c_void, *mut core::ffi::c_void) -> usize;

```
