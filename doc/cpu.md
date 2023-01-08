# Fox CPU

The Fox CPU is a Stack Machine.
It's a 32-bit system with native 32-bit words.

## Opcodes

### Table 

|      | 0     | 1    | 2     | 3     | 4    | 5    | 6    | 7    | 8    | 9    | A    | B    | C    | D    | E    | F    |
| ---- | ----- | ---- | ----- | ----- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| 0    | HALT  | DBG  |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 1    | LITW  | DUP  | DROP  | SWAP  | OVER | ROT  | LITB |      |      |      |      |      |      |      |      |      |
| 2    | ADD   | SUB  | MUL   | DIV   | AND  | OR   | XOR  | SHL  | SHR  | INC  | DEC  | SAR  | NOT  |      |      |      |
| 3    | SW    | LW   | SB    | LB    |      |      |      |      |      |      |      |      |      |      |      |      |
| 4    | EQU   | NEQ  | LT    | GT    | GTE  | LTE  |      |      |      |      |      |      |      |      |      |      |
| 5    | JMP   | JZ   | CALL  | RET   | JNZ  |      |      |      |      |      |      |      |      |      |      |      |
| 6    | RPUSH | RPOP | RPEEK | RDROP |      |      |      |      |      |      |      |      |      |      |      |      |
| 7    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 8    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| 9    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| A    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| B    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| C    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| D    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| E    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |
| F    |       |      |       |       |      |      |      |      |      |      |      |      |      |      |      |      |

### Detailed explanations
#### HALT (`0x00`)
This will halt the CPU waiting for a vector to trigger.

#### DBG (`0x01`)
This will debug print the contents of the stack and return stack.

#### LITW (`0x10`) [` -- a`]
This will read the next 4 bytes in little-endian format and put the value on the stack.

#### DUP (`0x11`) [`a -- a a`]
This will duplicate the top value on the stack.

#### DROP (`0x12`) [`a b -- a`]
This will drop the top value from the stack.

#### SWAP (`0x13`) [`a b -- b a`]
This will swap the top 2 values on the stack.

#### OVER (`0x14`) [`a b -- a b a`]
This will duplicate the value one down from the top.

#### ROT (`0x15`) [`a b c -- b c a`]
This will rotate the top 3 values.

#### LITB (`0x16`) [`-- a`]
This will read the next byte and put it zero-extended on the stack.

#### ADD (`0x20`) [`a b -- a+b`]
This will add the top 2 values on the stack together. It uses wrapping add.

#### SUB (`0x21`) [`a b -- a-b`]
This will subtract using the top 2 values on the stack. It uses wrapping sub.