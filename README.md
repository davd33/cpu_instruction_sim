# cpu_instruction_sim

## 8086 Instructions

```
MOV
1 0 0 0 1 0 D W                   # Reg/mem with reg
1 1 0 0 0 1 1 W | _ _ 0 0 0 _ _ _ # Immediate reg/mem
1 0 1 1 W _ _ _                   # Immediate to reg
1 0 1 0 0 0 0 W                   # Mem to acc
1 0 1 0 0 0 1 W                   # Acc to mem
ADD
0 0 0 0 0 0 D W                   # Reg/mem with reg 
1 0 0 0 0 0 S W | _ _ 0 0 0 _ _ _ # Immediate reg/mem
0 0 0 0 0 1 0 W                   # Immediate acc
SUB
0 0 1 0 1 0 D W                   # Reg/mem and reg
1 0 0 0 0 0 S W | _ _ 1 0 1 _ _ _ # Immediate from reg/mem
0 0 1 0 1 1 0 W                   # Immediate from acc
CMP
0 0 1 1 1 0 D W                   # Reg/mem and reg
1 0 0 0 0 0 S W | _ _ 1 1 1 _ _ _ # Immediate with reg/mem
0 0 1 1 1 1 0 W                   # Immediate with acc
```

## Test

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

For linux, you can also use the `test.sh` bash file (don't forget to build before).
