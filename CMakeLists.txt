cmake_minimum_required(VERSION 3.13)

link_directories(${CMAKE_SOURCE_DIR}/rust_module/target/thumbv6m-none-eabi/release)

include(pico_sdk_import.cmake)

project(test_project C CXX ASM)

find_package(Corrosion REQUIRED)

set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()
add_executable(firmware main.c)
pico_enable_stdio_uart(firmware 1)
pico_add_extra_outputs(firmware)
target_link_libraries(firmware hardware_adc pico_stdlib start_rust)

corrosion_import_crate(MANIFEST_PATH start_rust/Cargo.toml)
