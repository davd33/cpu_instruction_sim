#!/bin/bash

function test_asm() {
  echo "Test ${ASM_BINARY}..."
  ./target/debug/cpu-instruction-sim "${ASM_BINARY}" > output.txt 2> /dev/null

  # Then compile output with NASM:
  nasm output.txt

  # Then compare with diff:
  diff output "$ASM_BINARY" && echo "OK" || echo "KO"
}

function test_sim() {
  echo "Test ${ASM_BINARY}..."
  ./target/debug/cpu-instruction-sim "${ASM_BINARY}" --sim > output.txt 2> /dev/null

  # Then compare with diff:
  diff output.txt "${ASM_BINARY}.output" && echo "OK" || echo "KO"
}

# build executable
cargo build

# unit tests
cargo test

# disassembler tests
export ASM_BINARY='asm/mov_reg_to_reg/simple'
test_asm
export ASM_BINARY='asm/mov_reg_to_reg/long'
test_asm
export ASM_BINARY='asm/mov_mem_to_reg/simple'
test_asm
export ASM_BINARY='asm/mov_mem_to_reg/hard'
test_asm
export ASM_BINARY='asm/add_sub_cmp/simple'
test_asm

# cpu instructions simulation tests
export ASM_BINARY='asm/sim/immediate_movs'
test_sim
export ASM_BINARY='asm/sim/register_movs'
test_sim
