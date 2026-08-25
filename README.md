# deaffiner

affine crypto operations as simple as they get

```console
cargo b --release

./target/release/deaffiner encrypt ./data/The_Open_Window.txt -a 11 -b 17 | cipher.txt
./target/release/deaffiner crack ./cipher.txt
```
