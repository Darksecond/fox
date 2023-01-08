# Fox CPU

The Fox CPU is a Stack Machine.
It's a 32-bit system with native 32-bit words.
It has 16 megabytes of available memory, starting at 0x000.
The CPU will reset to 0x100 and start running from there.

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

#### MUL (`0x22`) [`a b -- a*b`]
This will multiply the top 2 values on the stack.

#### DIV (`0x23`) [`a b -- a/b`]
This will divide using the top 2 values on the stack.

#### AND (`0x24`) [`a b -- a&b`]
AND top 2 values on the stack.

#### OR (`0x25`) [`a b -- a|b`]
OR top 2 values on the stack.

#### XOR (`0x26`) [`a b -- a^b`]
XOR top 2 values on the stack.

#### SHL (`0x27`) [`a b -- a>>b`]
Shift Left.

#### SHR (`0x28`) [`a b -- a<<b`]
Shift Right.

#### INC (`0x29`) [`a -- a+1`]
Increment top value by one.

#### DEC (`0x2A`) [`a -- a-1`]
Decrement top value by one.

#### SAR (`0x2B`) [`a b -- a>>>b`]
Shift Arithmetic Right.

#### NOT (`0x2C`) [`a -- !a`]
NOT top value.

#### SW (`0x30`) [`value addr -- `]
Store top stack value as little-endian word at addr.

#### LW (`0x31`) [`addr -- value`]
Load addr as little-endian word from addr.

#### SB (`0x32`) [`value addr --`]
Store top stack value as byte at addr.

#### LB (`0x33`) [`addr -- value`]
Load byte from addr.

#### EQU (`0x40`) [`a b -- a==b`]
Compare top 2 values, pushes `1` if equal, `0` otherwise.

#### NEQ (`0x41`) [`a b -- a=!b`]
Compare top 2 values, pushes `1` if  not equal, `0` otherwise.

#### LT (`0x42`) [`a b -- a<b`]
Compare top 2 values, pushes `1` if a less than b, `0` otherwise.

#### GT (`0x43`) [`a b -- a>b`]
Compare top 2 values, pushes `1` if a greater than b, `0` otherwise.

#### LTE (`0x44`) [`a b -- a<=b`]
Compare top 2 values, pushes `1` if a less than or equal to b, `0` otherwise.

#### GTE (`0x45`) [`a b -- a>=b`]
Compare top 2 values, pushes `1` if a greater than or equal to b, `0` otherwise.

#### JMP (`0x50`) [`addr --`]
Unconditional jump to addr.

#### JZ (`0x51`) [`cond addr --`]
Jump if cond is `0` to addr.

#### CALL (`0x52`) [`addr --`]
Jump to addr, pushing return address onto the call stack.

#### RET (`0x52`) [`--`]
Return to addr, pops the top value from the call stack.

#### JNZ (`0x53`) [`cond addr --`]
Jump if cond is not `0`.

#### RPUSH (`0x60`) [`a --`]
Pushes top value to call stack.

#### RPOP (`0x61`) [`-- a`]
Pops top value from call stack.

#### RPEEK (`0x62`) [`-- a`]
Duplicates top value from call stack onto normal stack.
This is the same as doing: `RPOP DUP RPUSH`.

#### RDROP (`0x63`) [`--`]
Drops top value from call stack.
This is the same as doing: `RPOP DROP`.