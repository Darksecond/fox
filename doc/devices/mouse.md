# Mouse Device

The mouse device reports mouse activity in fox.
Vector is triggered on mouse move, state or button changes.

| Address    | Name         |
| ---------- | ------------ |
| 0x10050000 | Vector       |
| 0x10050004 | X Coordinate |
| 0x10050008 | Y Coordinate |
| 0x1005000C | Flags        |
| 0x10050010 | Button       |

## Flags

Flags are currently not implemented.

- Focus (`0x01`)

## Button

Left, Middle and Right mouse buttons are represented.

- Left (`0x01`)
- Middle (`0x02`)
- Right (`0x04`)
