# toolchain配置
rustup install nightly
rustup default nightly

# 安装target
rustup target add riscv32imac-unknown-none-elf

# 构建项目
cargo build

