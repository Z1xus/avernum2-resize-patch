# avernum2-resize-patch

lets avernum 2: crystal souls open a window as big as the screen, so [ihateborders](https://github.com/Z1xus/ihateborders) can make it borderless without black bars

the game limits its window to the screen less 100 by 80 pixels and never redraws for a new size. this patch removes the limit

## use

put `avernum2-resize-patch.exe` from [releases](https://github.com/Z1xus/avernum2-resize-patch/releases) in the game folder and run it

```text
[+] patched Avernum2.exe
[*] press enter to close
```

then set the game to windowed mode at the resolution of your screen, and make it borderless with "resize to screen" on

the original stays as `Avernum2.exe.bak`, rename back to undo

## build

```sh
cargo build --release
```
