# cpu_instruction_sim

To compare results between my 8086 instructions parser and NASM's compiled ones,
run the following commands in GIT Bash.

First run program with the asm binary file path as argument:

```bash
# change the path below to the file to be tested
export ASM_BINARY='/d/Code/cpu_instruction_sim/asm/mov_mem_to_reg/hard' 
/d/Code/cpu_instruction_sim/target/debug/cpu-instruction-sim.exe \
  "$ASM_BINARY" > /d/Code/cpu_instruction_sim/output.txt 2> /dev/null

# Then compile output with NASM:
cd /c/Program\ Files/NASM/
./nasm.exe /d/Code/cpu_instruction_sim/output.txt 

# Then compare with diff:
diff /d/Code/cpu_instruction_sim/output "$ASM_BINARY" 
```

Diff should not output any error.
