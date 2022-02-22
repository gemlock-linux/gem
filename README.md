# Gem
A shiny new package manager written in rust explicitly for gemlock/linux and it's distributions.

## List of content

* [How to setup](#how-to-setup)
    * Systems
        * [Ubuntu](#ubuntu)
        * [Arch](#arch)
    * [How to build](#how-to-build)
* [Modules](#modules)
* [License](#license)

## How to setup

To setup, it's quite simple. You install the rust toolchain from either your package manager or directly from the rust website linked [here](https://www.rust-lang.org/learn/get-started)

* [Rust](https://www.rust-lang.org/learn/get-started)
* [glibc](https://www.gnu.org/software/libc/)

### Ubuntu
```
sudo apt install curl build-essential -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Arch
```
sudo pacman -Sy base-devel curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## How to build

```sh
# To build, simpy run
~$ cargo build

# To run, simply do
~$ cargo run

# You can use each module by using
~$ cargo run -- [MODULE_NAME] <MODULE_ARGS>
```

## Modules
Those are the checklist for the modules / commands that are supported.
Note that not everything is implemented just yet, we're working on it!

- [ ] `build`
- [ ] `install`
- [ ] `uninstall`
- [ ] `kitchen`
- [ ] `strip`

## License

Gearbox's code is licensed under the [MIT licence](https://opensource.org/licenses/MIT). Please see [the licence file](./LICENSE) for more information. [tl;dr](https://tldrlegal.com/license/mit-license) you can do whatever you want as long as you include the original copyright and license notice in any copy of the software/source.