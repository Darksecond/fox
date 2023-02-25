# Fox Assembler

Fox comes with a assembler. It uses prefixes for anything not a instruction.
A local label is the same as `<label>/<local>`.

## Prefix Commands

| Prefix   | Example     | Result         | Description                     |
| -------- | ----------- | -------------- | ------------------------------- |
| `#`      | `#DEADBEEF` | `LIT DEADBEEF` | Literal u32                     |
| `.`      | `.DA`       | `DA`           | Raw u8                          |
| `@`      | `@asdf`     |                | Label                           |
| `;`      | `;asdf`     | `LIT <asdf>`   | Literal label reference         |
| `|`      | `|0100`     |                | Set absolute origin             |
| `""`     | `"asdf`     | `asdf`         | raw string                      |
| `$`      | `$4`        |                | Set relative origin             |
| `:`      | `:asdf`     | `<asdf>`       | Raw label reference             |
| `&`      | `&write`    |                | Local label                     |
| `;&`     | `;&write`   | `LIT <&write>` | Literal Local label reference   |
| `:&`     | `:&write`   | `LIT <&write>` | Raw Local label reference       |

