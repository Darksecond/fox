# Keyboard Device

The keyboard device will repsond to keypresses. Codepoint is a u32 unicode codepoint for the key pressed.
If a key has no corresponding unicode codepoint ( like arrow keys) it may be available in the buttons bitflags field.
Vector is triggered on key presses or releases.

| Address      | Name         |
| ------------ | ------------ |
| `0x10060000` | Vector       |
| `0x10060004` | Codepoint    |
| `0x10060008` | Button       |

## Button 

- Left  (`0x0001`)
- Right (`0x0002`)
- Up    (`0x0004`)
- Down  (`0x0008`)
