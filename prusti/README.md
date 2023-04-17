# Installing Prusti

1. Clone the repository first:

```sh
git clone https://github.com/viperproject/prusti-dev.git
```

2. `cd` to it and setup the necessary toolchains.

```sh
./x.py setup
```

3. build Prusti in `release` mode:

```sh
./x.py build release
```

4. Depackage the binary and its dependencies to some directory so that we can invoke `cargo-prusti` freely.

```sh
./x.py package [target_dir]
# Example: ./x.py package $HOME/.local/prusti
```

5. Add the installation path to your `PATH` environment variable and enjoy it!

## Special Notes

Interested users are recommended to read the book *Software Foundations* before playing with this verification tool.
