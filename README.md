# Yeet Transpiler
YEET transpiler for C and C++ file, written in Rust.

Transpile your C++ file into yeet code inspired by [this reddit post](https://www.reddit.com/r/ProgrammerHumor/comments/bgdxwn/yeet).

Examples output is in the [examples](https://github.com/SaltyAom/yeet-transpiler/tree/main/examples) folder.

![Fubuki](https://user-images.githubusercontent.com/35027979/119815992-5cad9e80-bf16-11eb-89f2-94d5e879fc9c.png)

## Usage
`yeett` the command to transpile your c++ code to yeet.

However, you will required to use gcc to compile .cpp file into binary.

Example usage:
```bash
# Output as main.yeet.cpp
yeett main.cpp

gcc main.yeet.cpp -o main
./main
```

## Installation
No package is published yet due to unstability of the projet.

You have to compile the binary from source (this repo).

## Prequisted
- Rust 1.51 or more
- Git

1. Using git clone, you can always have latest release.
```bash
git clone https://github.com/saltyAom/yeet-transpiler && cd yeet-transpiler
```

2. Compile the source using Rust
```bash
cargo build --bin --release yeett
```

3. Navigate to built folder
```bash
cd target/release

# In the folder there should have binary named `yeett`
./yeett
```

(For Linux and MacOS user)
4. We can install the binary to path and use it anywhere:
```bash
sudo cp yeett /usr/bin/yeett

yeet some_file.cpp
```

## Known limitation: 
- Support only 1 file for now.
- Hardcoded string cannot contains the following:
    - `;` 
    - `(` 
    - `)` 
    - `{` 
    - `}` 
    - `,` 
    - `&` 
    - `<<`
    - `>>`
    - `<`
    - `>`
    - `::`
    - `:`
    - `\"`
    - `[`
    - `]`
    - `++`
    - `--`
    - `return`

## Note
This project is made for pure meme intent and neither is stable nor having a long maintainance or long term support.

Please use this project at your own risk.
