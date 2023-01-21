# System Device

The system device contains sytem and misc. utilities.
Writing a value to `exit` will cause fox to exit with that status code.
Reading from `read` will read a `0` seperated, `0` terminated string, representing the command line arguments. Reading from the `read` port after all bytes have been read will result in a continuous `0`.

| Address      | Name         |
| ------------ | ------------ |
| `0x10010000` | Vector       |
| `0x10010004` | Exit         |
| `0x10010008` | Read         |