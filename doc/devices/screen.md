# Screen Device

The screen device allows for creating graphical interfaces, like programs and games.
Each layer has 512 kilobytes reserved, the actual space used is determined by the configured width and height.
The vector will trigger every frame.

| Address      | Name         |
| ------------ | ------------ |
| `0x10020000` | Vector       |
| `0x10020004` | Width        |
| `0x10020008` | Height       |
| `0x1002000C` | Cmd Length   |
| `0x10020010` | Cmd Addr     |
| `0x10020014` | Zoom         |
| `0x10020018` | Palette 0    |
| `0x10020054` | Palette 15   |

| Address      | Name         |
| ------------ | ------------ |
| `0x20000000` | Layer 0      |
| `0x20080000` | Layer 1      |
| `0x20100000` | Layer 2      |
| `0x20180000` | Layer 3      |

Writing to Command Address will execute the specified commands, make sure to set length first.

## Command

Byte 0x0F is for drawing bigger sprites. Setting it to 0x11 would for example draw a 16x16 sprite instead of a 8x8. Setting it to 0x20 would draw a 24x8 sprite and 0x01 would draw a 8x16 sprite. Sprites are drawn Left-to-Right then Top-to-Bottom.
Byte 0x0E is used in 1bpp mode for selecting the fore- and background colors.
The skip clear flag is used to skip 0 bytes in the sprite instead of inserting 0's. This can be used to overlay sprites on top of each other.
The command and layer nibbles are combined, the command lives in the higher nibble, the layer in the lower nibble.

Layout of a command:

| Address | Name        |
| ------- | ----------- |
| `0x00`  | X           |
| `0x04`  | Y           |
| `0x08`  | Source      |
| `0x0C`  | Cmd & Layer |
| `0x0D`  | Flags       |
| `0x0E`  | Color       |
| `0x0F`  | W & H       |


Commands:

| Value  | Name        |
| ------ | ----------- |
| `0x00` | Clear       |
| `0x01` | 1bpp Sprite |
| `0x02` | 4bpp Sprite |

Flags:

| Value  | Name       |
| ------ | ---------- |
| `0x01` | Flip X     |
| `0x02` | Flip Y     |
| `0x04` | Flip XY    |
| `0x08` | Skip Clear |
