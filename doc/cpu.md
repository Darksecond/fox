# Fox CPU

The Fox CPU is a Stack Machine.
It's a 32-bit system with native 32-bit words.

## Opcodes

|      | 0x?0  | 0x?1 | 0x?2  | 0x?3  | 0x?4 | 0x?5 | 0x?6 | 0x?7 | 0x?8 | 0x?9 | 0x?A | 0x?B | 0x?C | 0x?D | 0x?E | 0x?F |
| ---- | ----- | ---- | ----- | ----- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| 0x0? | HALT  | DBG  |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0x1? | LITW  | DUP  | DROP  | SWAP  | OVER | ROT  | LITB |      |      |      |      |      |      |      |      |      |
| 0x2? | ADD   | SUB  | MUL   | DIV   | AND  | OR   | XOR  | SHL  | SHR  | INC  | DEC  | SAR  | NOT  |      |      |      |
| 0x3? | SW    | LW   | SB    | LB    |      |      |      |      |      |      |      |      |      |      |      |      |
| 0x4? | EQU   | NEQ  | LT    | GT    | GTE  | LTE  |      |      |      |      |      |      |      |      |      |      |
| 0x5? | JMP   | JZ   | CALL  | RET   | JNZ  |      |      |      |      |      |      |      |      |      |      |      |
| 0x6? | RPUSH | RPOP | RPEEK | RDROP |      |      |      |      |      |      |      |      |      |      |      |      |
| 0x7? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0x8? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0x9? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xA? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xB? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xC? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xD? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xE? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 0xF? |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |