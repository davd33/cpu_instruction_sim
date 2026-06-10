#!/bin/bash

function test_asm() {
  echo "Test ${ASM_BINARY}..."
  ./target/debug/cpu-instruction-sim "${ASM_BINARY}" > output.txt 2> /dev/null

  # Then compile output with NASM:
  nasm output.txt

  # Then compare with diff:
  diff output "$ASM_BINARY" && echo "OK" || echo "KO"
}

# build executable
cargo build

# change the path below to the file to be tested
export ASM_BINARY='asm/mov_reg_to_reg/simple'
test_asm
export ASM_BINARY='asm/mov_reg_to_reg/long'
test_asm
export ASM_BINARY='asm/mov_mem_to_reg/simple'
test_asm
export ASM_BINARY='asm/mov_mem_to_reg/hard'
test_asm
