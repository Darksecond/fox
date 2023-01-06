# Fox CPU

The Fox CPU is a Stack Machine.
It's a 32-bit system with native 32-bit words.

## Opcodes

|      | ?0    | ?1   | ?2    | ?3    | ?4   | ?5   | ?6   | ?7   | ?8   | ?9   | ?A   | ?B   | ?C   | ?D   | ?E   | ?F   |
| ---- | ----- | ---- | ----- | ----- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| 0?   | HALT  | DBG  |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 1?   | LITW  | DUP  | DROP  | SWAP  | OVER | ROT  | LITB |      |      |      |      |      |      |      |      |      |
| 2?   | ADD   | SUB  | MUL   | DIV   | AND  | OR   | XOR  | SHL  | SHR  | INC  | DEC  | SAR  | NOT  |      |      |      |
| 3?   | SW    | LW   | SB    | LB    |      |      |      |      |      |      |      |      |      |      |      |      |
| 4?   | EQU   | NEQ  | LT    | GT    | GTE  | LTE  |      |      |      |      |      |      |      |      |      |      |
| 5?   | JMP   | JZ   | CALL  | RET   | JNZ  |      |      |      |      |      |      |      |      |      |      |      |
| 6?   | RPUSH | RPOP | RPEEK | RDROP |      |      |      |      |      |      |      |      |      |      |      |      |
| 7?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 8?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 9?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| A?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| B?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| C?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| D?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| E?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| F?   |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |