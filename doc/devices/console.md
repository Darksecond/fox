# Console Device

The console device allows for reading and writing to standard console input and output.

| Address      | Name         |
| ------------ | ------------ |
| `0x10000000` | Vector       |
| `0x10000004` | Write        |
| `0x10000008` | Read         |
| `0x1000000C` | Error        |

## Output

Writing to the `write` or `error` ports will immediate output to the corresponding output (stdout and stderr). No unicode transformations are done and it output the raw bytes.

## Input

Vector will trigger once for every byte on stdin. Read from the `read` port to get the byte. A new byte will be put on the port once the vector triggers again. This means that reading twice in a single interrupt call will result in the same value twice.