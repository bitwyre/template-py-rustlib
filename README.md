# Rust Library Template for Python

## Template TODO

- [ ] Make this template repo also for C/C++ Library
- [ ] Automate build with CI/CD

## Template Demo

Ensure you're using Python3.7 or later

```bash
./rebuild-rinstall.sh debug
python
```

then

```python
>>> from py_rustlib.py_rustlib import say_hello_to
>>> say_hello_to("Bitwyre")
"Hello Bitwyre! I'm 🦀!"
>>>
```

## Fresh Repo TODO

1. Update [Cargo.toml](Cargo.toml)
2. Update [setup.py](setup.py)
3. Update [README.md](README.md)
4. Update [LICENSE](LICENSE) (if necessary)
5. Update the code in [src/lib.rs](src/lib.rs)

## Pre-requisites

- Linux/Mac
- Nightly Rust
- Python 3.7 or later
- [cbindgen](https://github.com/eqrion/cbindgen) (for C/C++ not yet implemented)

## Installing the package

### Debug Build & Install (Python Only)

```bash
./rebuild-install.sh debug
```

### Release Build & Install (Python Only)

```bash
./rebuild-install.sh
```

## Author

[Aditya Kresna](https://github.com/ujang360)

## Copyright

&copy; 2020 Bitwyre Technologies LLC
