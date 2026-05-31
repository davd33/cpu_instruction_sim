#!/bin/bash

# change the path below to the file to be tested
export ASM_BINARY='asm/mov_mem_to_reg/hard'
./target/debug/cpu-instruction-sim "$ASM_BINARY" > output.txt 2> /dev/null

# Then compile output with NASM:
nasm output.txt

# Then compare with diff:
diff output "$ASM_BINARY"