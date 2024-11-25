# cwcp - Current Working Directory to Clipboard
So I was so lazy to run `pwd | pbcopy` to copy the current working directory to the clipboard in my macos environment computer. So I wrote a simple rust program to do that. I could also define alias in my shell config but if there is a way to do it in rust, why not?

> [!WARNING]
> This program only works on MacOS. Maybe in the future I can add Linux support as well, who knows?

## Installation
```bash
cargo install cwcp --git https://github.com/pehlicd/cwcp 
```

or just clone the repo and run build.