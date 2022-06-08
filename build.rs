extern crate cc;

fn main() {
    cc::Build::new()
        .compiler("riscv64-unknown-elf-gcc")
        .file("src/portable/portASM.S")
        .file("src/kernel/config_resolve.c")
        .file("src/ffi/main_blinky.c")
        .file("src/ffi/riscv-virt.c")
        .file("src/tests/td_task.c")
        .file("src/tests/test.c")
        .file("src/tests/queue/queue_receive_blocking_utest.c")
        .file("src/tests/queue/queue_receive_nonblocking_utest.c")
        .file("src/tests/queue/queue_reset_utest.c")
        .file("src/tests/queue/queue_send_blocking_utest.c")
        .file("src/tests/queue/queue_send_nonblocking_utest.c")
        .file("src/tests/queue/queue_status_utest.c")
        .file("src/tests/queue/queue_delete_dynamic_utest.c")
        .file("src/tests/queue/queue_delete_static_utest.c")
        .file("src/tests/queue/queue_create_dynamic_utest.c")
        .file("src/tests/queue/queue_create_static_utest.c")
        .file("src/tests/queue/binary_semaphore_utest.c")
        .file("src/tests/queue/counting_semaphore_utest.c")
        .file("src/tests/queue/semaphore_common_utest.c")
        .file("src/tests/queue/semaphore_create_utest.c")
        .flag("-march=rv32ima")
        .compile("portASM");
}
