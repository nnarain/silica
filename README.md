# Silica

[![Build Status](https://travis-ci.org/nnarain/silica.svg?branch=master)](https://travis-ci.org/nnarain/silica)
[![codecov](https://codecov.io/gh/nnarain/silica/branch/master/graph/badge.svg)](https://codecov.io/gh/nnarain/silica)

Chip8 Assembler written in the Rust programming language.

Usage
-----

```
silica -o output.c8 <myfile.asm>
```

Build
-----

```
cargo build
```

Example
-------

Here is a simple example program written in Chip8 assembly. This program will print the numbers 0, 1, 2

```asm

            org $200

start       LD I, #num0
            LD V0, 0
            LD V1, 0

            DRW V0, V1, 5

            LD I, #num1
            LD V0, 10
            LD V1, 0
            
            DRW V0, V1, 5

            LD I, #num2
            LD V0, 20
            LD V1, 0
            
            DRW V0, V1, 5

            LD I, #num3
            LD V0, 30
            LD V1, 0
            
            DRW V0, V1, 5

end         JP #end          ; loop forever

; Sprites
num0
            db $F0 $90 $90 $90 $F0
num1
            db $20 $60 $20 $20 $70
num2
            db $F0 $10 $F0 $80 $F0
num3
            db $F0 $10 $F0 $10 $F0


```