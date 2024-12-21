This is a small demo project that implements a simple JIT for evaluating arithmetic expressions.
It assumes it is running under Linux on x86_64.
It uses 64 bit signed numbers and supports integer addition, subtraction, multiplication and division.

# How to build and run

Provide the expression to be evaluated on the commandline like so:

```
$ cargo run "2+3 *4 + 10/5 - 6*(9+7)"
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/jit-expression-eval '2+3 *4 + 10/5 - 6*(9+7)'`
AST: (((2 + (3 * 4)) + (10 / 5)) - (6 * (9 + 7)))
res_interpreter: -80
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 02  00 00 00 00 00 00 00 50 
0x0000000010: 48 b8 03 00 00 00 00 00  00 00 50 48 b8 04 00 00 
0x0000000020: 00 00 00 00 00 50 5b 58  48 f7 eb 50 5b 58 48 01 
0x0000000030: d8 50 48 b8 0a 00 00 00  00 00 00 00 50 48 b8 05 
0x0000000040: 00 00 00 00 00 00 00 50  5b 58 48 99 48 f7 fb 50 
0x0000000050: 5b 58 48 01 d8 50 48 b8  06 00 00 00 00 00 00 00 
0x0000000060: 50 48 b8 09 00 00 00 00  00 00 00 50 48 b8 07 00 
0x0000000070: 00 00 00 00 00 00 50 5b  58 48 01 d8 50 5b 58 48 
0x0000000080: f7 eb 50 5b 58 48 29 d8  50 58 5b 5d c3 
res_jit:         -80
```

# How to test

The included unit tests randomly generate some arithmetic expressions,
compute the result both by interpreting the resulting syntax tree and by jitting it down to native machine code and running that,
and assert that both results agree with each other.
One example run:

```
$ cargo test -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests src/main.rs (target/debug/deps/jit_expression_eval-13417cf070e179bf)

running 1 test
Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (((-604 / 1056) * -1350) * ((1522 * (-1247 / (2287 - -2422))) * 240))
res_interpreter: 0
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 a4  fd ff ff ff ff ff ff 50 
0x0000000010: 48 b8 20 04 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 48 b8 ba fa ff  ff ff ff ff ff 50 5b 58 
0x0000000030: 48 f7 eb 50 48 b8 f2 05  00 00 00 00 00 00 50 48 
0x0000000040: b8 21 fb ff ff ff ff ff  ff 50 48 b8 ef 08 00 00 
0x0000000050: 00 00 00 00 50 48 b8 8a  f6 ff ff ff ff ff ff 50 
0x0000000060: 5b 58 48 29 d8 50 5b 58  48 99 48 f7 fb 50 5b 58 
0x0000000070: 48 f7 eb 50 48 b8 f0 00  00 00 00 00 00 00 50 5b 
0x0000000080: 58 48 f7 eb 50 5b 58 48  f7 eb 50 58 5b 5d c3 
res_jit:         0

AST: (3719 / 1857)
res_interpreter: 2
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 87  0e 00 00 00 00 00 00 50 
0x0000000010: 48 b8 41 07 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 58 5b 5d c3 
res_jit:         2

AST: (((654 * 1951) * (3063 - 1866)) * 1342)
res_interpreter: 2049659330796
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 8e  02 00 00 00 00 00 00 50 
0x0000000010: 48 b8 9f 07 00 00 00 00  00 00 50 5b 58 48 f7 eb 
0x0000000020: 50 48 b8 f7 0b 00 00 00  00 00 00 50 48 b8 4a 07 
0x0000000030: 00 00 00 00 00 00 50 5b  58 48 29 d8 50 5b 58 48 
0x0000000040: f7 eb 50 48 b8 3e 05 00  00 00 00 00 00 50 5b 58 
0x0000000050: 48 f7 eb 50 58 5b 5d c3 
res_jit:         2049659330796

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (((-1522 * (2035 / -1110)) * (-1452 - ((-3377 / -366) - -673))) - -697)
res_interpreter: -3247251
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 0e  fa ff ff ff ff ff ff 50 
0x0000000010: 48 b8 f3 07 00 00 00 00  00 00 50 48 b8 aa fb ff 
0x0000000020: ff ff ff ff ff 50 5b 58  48 99 48 f7 fb 50 5b 58 
0x0000000030: 48 f7 eb 50 48 b8 54 fa  ff ff ff ff ff ff 50 48 
0x0000000040: b8 cf f2 ff ff ff ff ff  ff 50 48 b8 92 fe ff ff 
0x0000000050: ff ff ff ff 50 5b 58 48  99 48 f7 fb 50 48 b8 5f 
0x0000000060: fd ff ff ff ff ff ff 50  5b 58 48 29 d8 50 5b 58 
0x0000000070: 48 29 d8 50 5b 58 48 f7  eb 50 48 b8 47 fd ff ff 
0x0000000080: ff ff ff ff 50 5b 58 48  29 d8 50 58 5b 5d c3 
res_jit:         -3247251

AST: ((-492 - 2624) * -2592)
res_interpreter: 8076672
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 14  fe ff ff ff ff ff ff 50 
0x0000000010: 48 b8 40 0a 00 00 00 00  00 00 50 5b 58 48 29 d8 
0x0000000020: 50 48 b8 e0 f5 ff ff ff  ff ff ff 50 5b 58 48 f7 
0x0000000030: eb 50 58 5b 5d c3 
res_jit:         8076672

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (3919 / -655)
res_interpreter: -5
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 4f  0f 00 00 00 00 00 00 50 
0x0000000010: 48 b8 71 fd ff ff ff ff  ff ff 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 58 5b 5d c3 
res_jit:         -5

AST: ((-2691 + 284) + (3543 - -3231))
res_interpreter: 4367
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 7d  f5 ff ff ff ff ff ff 50 
0x0000000010: 48 b8 1c 01 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000020: 50 48 b8 d7 0d 00 00 00  00 00 00 50 48 b8 61 f3 
0x0000000030: ff ff ff ff ff ff 50 5b  58 48 29 d8 50 5b 58 48 
0x0000000040: 01 d8 50 58 5b 5d c3 
res_jit:         4367

AST: (-2730 / -3491)
res_interpreter: 0
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 56  f5 ff ff ff ff ff ff 50 
0x0000000010: 48 b8 5d f2 ff ff ff ff  ff ff 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 58 5b 5d c3 
res_jit:         0

AST: ((-2857 * (-243 + -1880)) - 1420)
res_interpreter: 6063991
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 d7  f4 ff ff ff ff ff ff 50 
0x0000000010: 48 b8 0d ff ff ff ff ff  ff ff 50 48 b8 a8 f8 ff 
0x0000000020: ff ff ff ff ff 50 5b 58  48 01 d8 50 5b 58 48 f7 
0x0000000030: eb 50 48 b8 8c 05 00 00  00 00 00 00 50 5b 58 48 
0x0000000040: 29 d8 50 58 5b 5d c3 
res_jit:         6063991

AST: (3600 * (-3265 * ((((-1177 * 2549) * 2945) / -2958) / 3104)))
res_interpreter: -11307348000
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 10  0e 00 00 00 00 00 00 50 
0x0000000010: 48 b8 3f f3 ff ff ff ff  ff ff 50 48 b8 67 fb ff 
0x0000000020: ff ff ff ff ff 50 48 b8  f5 09 00 00 00 00 00 00 
0x0000000030: 50 5b 58 48 f7 eb 50 48  b8 81 0b 00 00 00 00 00 
0x0000000040: 00 50 5b 58 48 f7 eb 50  48 b8 72 f4 ff ff ff ff 
0x0000000050: ff ff 50 5b 58 48 99 48  f7 fb 50 48 b8 20 0c 00 
0x0000000060: 00 00 00 00 00 50 5b 58  48 99 48 f7 fb 50 5b 58 
0x0000000070: 48 f7 eb 50 5b 58 48 f7  eb 50 58 5b 5d c3 
res_jit:         -11307348000

AST: (1838 * -243)
res_interpreter: -446634
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 2e  07 00 00 00 00 00 00 50 
0x0000000010: 48 b8 0d ff ff ff ff ff  ff ff 50 5b 58 48 f7 eb 
0x0000000020: 50 58 5b 5d c3 
res_jit:         -446634

AST: (((1030 / ((((((((981 - 2446) + -1407) - -2569) * -2519) + -2704) - ((2813 + (-2021 - 1080)) - (645 * 1202))) - ((1648 - -2379) - ((-12 / ((-1371 - (2679 + 3216)) / 1026)) / 2933))) - -1936)) + (2969 / 1486)) - 1072)
res_interpreter: -1071
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 06  04 00 00 00 00 00 00 50 
0x0000000010: 48 b8 d5 03 00 00 00 00  00 00 50 48 b8 8e 09 00 
0x0000000020: 00 00 00 00 00 50 5b 58  48 29 d8 50 48 b8 81 fa 
0x0000000030: ff ff ff ff ff ff 50 5b  58 48 01 d8 50 48 b8 f7 
0x0000000040: f5 ff ff ff ff ff ff 50  5b 58 48 29 d8 50 48 b8 
0x0000000050: 29 f6 ff ff ff ff ff ff  50 5b 58 48 f7 eb 50 48 
0x0000000060: b8 70 f5 ff ff ff ff ff  ff 50 5b 58 48 01 d8 50 
0x0000000070: 48 b8 fd 0a 00 00 00 00  00 00 50 48 b8 1b f8 ff 
0x0000000080: ff ff ff ff ff 50 48 b8  38 04 00 00 00 00 00 00 
0x0000000090: 50 5b 58 48 29 d8 50 5b  58 48 01 d8 50 48 b8 85 
0x00000000a0: 02 00 00 00 00 00 00 50  48 b8 b2 04 00 00 00 00 
0x00000000b0: 00 00 50 5b 58 48 f7 eb  50 5b 58 48 29 d8 50 5b 
0x00000000c0: 58 48 29 d8 50 48 b8 70  06 00 00 00 00 00 00 50 
0x00000000d0: 48 b8 b5 f6 ff ff ff ff  ff ff 50 5b 58 48 29 d8 
0x00000000e0: 50 48 b8 f4 ff ff ff ff  ff ff ff 50 48 b8 a5 fa 
0x00000000f0: ff ff ff ff ff ff 50 48  b8 77 0a 00 00 00 00 00 
0x0000000100: 00 50 48 b8 90 0c 00 00  00 00 00 00 50 5b 58 48 
0x0000000110: 01 d8 50 5b 58 48 29 d8  50 48 b8 02 04 00 00 00 
0x0000000120: 00 00 00 50 5b 58 48 99  48 f7 fb 50 5b 58 48 99 
0x0000000130: 48 f7 fb 50 48 b8 75 0b  00 00 00 00 00 00 50 5b 
0x0000000140: 58 48 99 48 f7 fb 50 5b  58 48 29 d8 50 5b 58 48 
0x0000000150: 29 d8 50 48 b8 70 f8 ff  ff ff ff ff ff 50 5b 58 
0x0000000160: 48 29 d8 50 5b 58 48 99  48 f7 fb 50 48 b8 99 0b 
0x0000000170: 00 00 00 00 00 00 50 48  b8 ce 05 00 00 00 00 00 
0x0000000180: 00 50 5b 58 48 99 48 f7  fb 50 5b 58 48 01 d8 50 
0x0000000190: 48 b8 30 04 00 00 00 00  00 00 50 5b 58 48 29 d8 
0x00000001a0: 50 58 5b 5d c3 
res_jit:         -1071

AST: (972 * (((-1921 / -3921) / 530) + 1272))
res_interpreter: 1236384
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 cc  03 00 00 00 00 00 00 50 
0x0000000010: 48 b8 7f f8 ff ff ff ff  ff ff 50 48 b8 af f0 ff 
0x0000000020: ff ff ff ff ff 50 5b 58  48 99 48 f7 fb 50 48 b8 
0x0000000030: 12 02 00 00 00 00 00 00  50 5b 58 48 99 48 f7 fb 
0x0000000040: 50 48 b8 f8 04 00 00 00  00 00 00 50 5b 58 48 01 
0x0000000050: d8 50 5b 58 48 f7 eb 50  58 5b 5d c3 
res_jit:         1236384

AST: (2536 / 1630)
res_interpreter: 1
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 e8  09 00 00 00 00 00 00 50 
0x0000000010: 48 b8 5e 06 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 58 5b 5d c3 
res_jit:         1

AST: (-1338 / 1927)
res_interpreter: 0
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 c6  fa ff ff ff ff ff ff 50 
0x0000000010: 48 b8 87 07 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 58 5b 5d c3 
res_jit:         0

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (((939 - -75) * (2279 - (-4093 - -1831))) / (1319 - ((-2548 * (-329 / 3662)) * 947)))
res_interpreter: 3490
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 ab  03 00 00 00 00 00 00 50 
0x0000000010: 48 b8 b5 ff ff ff ff ff  ff ff 50 5b 58 48 29 d8 
0x0000000020: 50 48 b8 e7 08 00 00 00  00 00 00 50 48 b8 03 f0 
0x0000000030: ff ff ff ff ff ff 50 48  b8 d9 f8 ff ff ff ff ff 
0x0000000040: ff 50 5b 58 48 29 d8 50  5b 58 48 29 d8 50 5b 58 
0x0000000050: 48 f7 eb 50 48 b8 27 05  00 00 00 00 00 00 50 48 
0x0000000060: b8 0c f6 ff ff ff ff ff  ff 50 48 b8 b7 fe ff ff 
0x0000000070: ff ff ff ff 50 48 b8 4e  0e 00 00 00 00 00 00 50 
0x0000000080: 5b 58 48 99 48 f7 fb 50  5b 58 48 f7 eb 50 48 b8 
0x0000000090: b3 03 00 00 00 00 00 00  50 5b 58 48 f7 eb 50 5b 
0x00000000a0: 58 48 29 d8 50 5b 58 48  99 48 f7 fb 50 58 5b 5d 
0x00000000b0: c3 
res_jit:         3490

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (-1073 + (52 / ((((3634 - 533) / ((904 - -2818) * (-3364 * 3874))) - (1208 / 1163)) + 3298)))
res_interpreter: -1073
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 cf  fb ff ff ff ff ff ff 50 
0x0000000010: 48 b8 34 00 00 00 00 00  00 00 50 48 b8 32 0e 00 
0x0000000020: 00 00 00 00 00 50 48 b8  15 02 00 00 00 00 00 00 
0x0000000030: 50 5b 58 48 29 d8 50 48  b8 88 03 00 00 00 00 00 
0x0000000040: 00 50 48 b8 fe f4 ff ff  ff ff ff ff 50 5b 58 48 
0x0000000050: 29 d8 50 48 b8 dc f2 ff  ff ff ff ff ff 50 48 b8 
0x0000000060: 22 0f 00 00 00 00 00 00  50 5b 58 48 f7 eb 50 5b 
0x0000000070: 58 48 f7 eb 50 5b 58 48  99 48 f7 fb 50 48 b8 b8 
0x0000000080: 04 00 00 00 00 00 00 50  48 b8 8b 04 00 00 00 00 
0x0000000090: 00 00 50 5b 58 48 99 48  f7 fb 50 5b 58 48 29 d8 
0x00000000a0: 50 48 b8 e2 0c 00 00 00  00 00 00 50 5b 58 48 01 
0x00000000b0: d8 50 5b 58 48 99 48 f7  fb 50 5b 58 48 01 d8 50 
0x00000000c0: 58 5b 5d c3 
res_jit:         -1073

AST: ((520 / (3865 / 509)) - (696 * (-3513 + 3723)))
res_interpreter: -146086
JIT CODE: 0x0000000000: 55 48 89 e5 53 48 b8 08  02 00 00 00 00 00 00 50 
0x0000000010: 48 b8 19 0f 00 00 00 00  00 00 50 48 b8 fd 01 00 
0x0000000020: 00 00 00 00 00 50 5b 58  48 99 48 f7 fb 50 5b 58 
0x0000000030: 48 99 48 f7 fb 50 48 b8  b8 02 00 00 00 00 00 00 
0x0000000040: 50 48 b8 47 f2 ff ff ff  ff ff ff 50 48 b8 8b 0e 
0x0000000050: 00 00 00 00 00 00 50 5b  58 48 01 d8 50 5b 58 48 
0x0000000060: f7 eb 50 5b 58 48 29 d8  50 58 5b 5d c3 
res_jit:         -146086

test tests::test_compare_interpreter_jit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
