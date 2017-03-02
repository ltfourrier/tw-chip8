tw-chip8
========

# What is this?

tw-chip8 is a chip-8 emulator/interpreter/disassembler
I made for fun (and educational purposes!).  
It currently only supports basic, non-ETI 660 chip-8 ROMs.

# Usage

./tw-chip8 [--disassemble | [--run] [--dump=DUMP_FILE]]  
The --dump option is used to dump the memory after the emulator ends.

# Additional information

Because the interpreter doesn't live in the emulated memory, the
0x0000 - 0x0200 address space contains a few useful virtual routines that can
be executed with a SYS call for debugging purposes:

- SYS 0x100: exit the interpreter and dump the memory, if using --dump

Remember to remove those calls after debug for maximum compatibility.