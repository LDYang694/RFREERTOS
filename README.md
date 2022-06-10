# RFREERTOS

FreeRTOS implemented in Rust





## Environment Prepare

##### Environment needed

- rust environment 
- Riscv toolchain
- qemu
- if you want to build Image run on D1，download T-head Riscv toolchain in [occ.t-head.cn](https://occ.t-head.cn/community/download).

##### toolchain prepare

```shell
rustup install nightly
rustup default nightly
```

##### target install

```shell
rustup target add riscv32imac-unknown-none-elf
rustup target add riscv64imac-unknown-none-elf
rustup target add riscv64gc-unknown-none-elf
```



## Build

#### qemu

We provide `Makefile` to help quickly start build 

- if you want to build project that fit qemu-system-riscv32,set config in `./.cargo/config`

```
[build]
target = "riscv32imac-unknown-none-elf"
```

and run `make run32`

- else if you want to build project that fit qemu-system-riscv64,set config in`./.cargo/config`

```
[build]
target = "riscv64imac-unknown-none-elf"
```

and run `make run64` 

#### D1

if you want to build project that can run on the D1 board,please checkout to `master-64D1` branch

```
make build
```

and you can find `RFreeRTOS_Image` file. You can just load that image file run on the D1 board without any other helper process like `OpenSBI`.



## DOC

You can find some sample docs in [RFREERTOS_DOC](https://ldyang694.github.io/RFREERTOS/r_freertos/index.html).





## Acknowledgement

This is a group experiment project of THU OS course. Thank teachers, teaching assistants and engineers for their support and help.

Due to limited time and capacity, there may still be many problems and incomplete documents in the project. If you are interested in this project and find some problems, welcome to [file an issue](https://github.com/LDYang694/RFREERTOS/issues).



