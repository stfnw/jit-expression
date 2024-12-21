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
AST: (363 + 2194)
res_interpreter: 2557
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 6b  01 00 00 00 00 00 00 50 
0x0000000010: 48 b8 92 08 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000020: 50 58 5b 5d c3 
res_jit:         2557

AST: ((-469 * (-1879 - (999 + -296))) / -899)
res_interpreter: -1347
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 2b  fe ff ff ff ff ff ff 50 
0x0000000010: 48 b8 a9 f8 ff ff ff ff  ff ff 50 48 b8 e7 03 00 
0x0000000020: 00 00 00 00 00 50 48 b8  d8 fe ff ff ff ff ff ff 
0x0000000030: 50 5b 58 48 01 d8 50 5b  58 48 29 d8 50 5b 58 48 
0x0000000040: f7 eb 50 48 b8 7d fc ff  ff ff ff ff ff 50 5b 58 
0x0000000050: 48 99 48 f7 fb 50 58 5b  5d c3 
res_jit:         -1347

AST: ((((((3730 / (((-562 / 1098) + (3858 + (720 - -1409))) / -976)) / 2756) * ((3683 + ((-638 - -2798) + ((-2623 - 120) + (3970 * 2940)))) - (-2331 * 2645))) - 3048) - -1214) - (-2325 * (3017 + 2292)))
res_interpreter: 12341591
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 92  0e 00 00 00 00 00 00 50 
0x0000000010: 48 b8 ce fd ff ff ff ff  ff ff 50 48 b8 4a 04 00 
0x0000000020: 00 00 00 00 00 50 5b 58  48 99 48 f7 fb 50 48 b8 
0x0000000030: 12 0f 00 00 00 00 00 00  50 48 b8 d0 02 00 00 00 
0x0000000040: 00 00 00 50 48 b8 7f fa  ff ff ff ff ff ff 50 5b 
0x0000000050: 58 48 29 d8 50 5b 58 48  01 d8 50 5b 58 48 01 d8 
0x0000000060: 50 48 b8 30 fc ff ff ff  ff ff ff 50 5b 58 48 99 
0x0000000070: 48 f7 fb 50 5b 58 48 99  48 f7 fb 50 48 b8 c4 0a 
0x0000000080: 00 00 00 00 00 00 50 5b  58 48 99 48 f7 fb 50 48 
0x0000000090: b8 63 0e 00 00 00 00 00  00 50 48 b8 82 fd ff ff 
0x00000000a0: ff ff ff ff 50 48 b8 12  f5 ff ff ff ff ff ff 50 
0x00000000b0: 5b 58 48 29 d8 50 48 b8  c1 f5 ff ff ff ff ff ff 
0x00000000c0: 50 48 b8 78 00 00 00 00  00 00 00 50 5b 58 48 29 
0x00000000d0: d8 50 48 b8 82 0f 00 00  00 00 00 00 50 48 b8 7c 
0x00000000e0: 0b 00 00 00 00 00 00 50  5b 58 48 f7 eb 50 5b 58 
0x00000000f0: 48 01 d8 50 5b 58 48 01  d8 50 5b 58 48 01 d8 50 
0x0000000100: 48 b8 e5 f6 ff ff ff ff  ff ff 50 48 b8 55 0a 00 
0x0000000110: 00 00 00 00 00 50 5b 58  48 f7 eb 50 5b 58 48 29 
0x0000000120: d8 50 5b 58 48 f7 eb 50  48 b8 e8 0b 00 00 00 00 
0x0000000130: 00 00 50 5b 58 48 29 d8  50 48 b8 42 fb ff ff ff 
0x0000000140: ff ff ff 50 5b 58 48 29  d8 50 48 b8 eb f6 ff ff 
0x0000000150: ff ff ff ff 50 48 b8 c9  0b 00 00 00 00 00 00 50 
0x0000000160: 48 b8 f4 08 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000170: 50 5b 58 48 f7 eb 50 5b  58 48 29 d8 50 58 5b 5d 
0x0000000180: c3 
res_jit:         12341591

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (((-257 - 4063) + 2727) + (-3793 * 804))
res_interpreter: -3051165
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 ff  fe ff ff ff ff ff ff 50 
0x0000000010: 48 b8 df 0f 00 00 00 00  00 00 50 5b 58 48 29 d8 
0x0000000020: 50 48 b8 a7 0a 00 00 00  00 00 00 50 5b 58 48 01 
0x0000000030: d8 50 48 b8 2f f1 ff ff  ff ff ff ff 50 48 b8 24 
0x0000000040: 03 00 00 00 00 00 00 50  5b 58 48 f7 eb 50 5b 58 
0x0000000050: 48 01 d8 50 58 5b 5d c3 
res_jit:         -3051165

AST: (-1084 - -1181)
res_interpreter: 97
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 c4  fb ff ff ff ff ff ff 50 
0x0000000010: 48 b8 63 fb ff ff ff ff  ff ff 50 5b 58 48 29 d8 
0x0000000020: 50 58 5b 5d c3 
res_jit:         97

AST: (1595 + 288)
res_interpreter: 1883
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 3b  06 00 00 00 00 00 00 50 
0x0000000010: 48 b8 20 01 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000020: 50 58 5b 5d c3 
res_jit:         1883

AST: (((-3857 - -3212) + ((2377 - ((-1009 - ((-3193 - 1334) + ((-3049 + (-1202 + ((-504 - (((3994 + 340) - ((2312 - (-210 + 841)) / 896)) + 3745)) / 2524))) * 2112))) * -1601)) * -3795)) - -1455)
res_interpreter: -54609047902875
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 ef  f0 ff ff ff ff ff ff 50 
0x0000000010: 48 b8 74 f3 ff ff ff ff  ff ff 50 5b 58 48 29 d8 
0x0000000020: 50 48 b8 49 09 00 00 00  00 00 00 50 48 b8 0f fc 
0x0000000030: ff ff ff ff ff ff 50 48  b8 87 f3 ff ff ff ff ff 
0x0000000040: ff 50 48 b8 36 05 00 00  00 00 00 00 50 5b 58 48 
0x0000000050: 29 d8 50 48 b8 17 f4 ff  ff ff ff ff ff 50 48 b8 
0x0000000060: 4e fb ff ff ff ff ff ff  50 48 b8 08 fe ff ff ff 
0x0000000070: ff ff ff 50 48 b8 9a 0f  00 00 00 00 00 00 50 48 
0x0000000080: b8 54 01 00 00 00 00 00  00 50 5b 58 48 01 d8 50 
0x0000000090: 48 b8 08 09 00 00 00 00  00 00 50 48 b8 2e ff ff 
0x00000000a0: ff ff ff ff ff 50 48 b8  49 03 00 00 00 00 00 00 
0x00000000b0: 50 5b 58 48 01 d8 50 5b  58 48 29 d8 50 48 b8 80 
0x00000000c0: 03 00 00 00 00 00 00 50  5b 58 48 99 48 f7 fb 50 
0x00000000d0: 5b 58 48 29 d8 50 48 b8  a1 0e 00 00 00 00 00 00 
0x00000000e0: 50 5b 58 48 01 d8 50 5b  58 48 29 d8 50 48 b8 dc 
0x00000000f0: 09 00 00 00 00 00 00 50  5b 58 48 99 48 f7 fb 50 
0x0000000100: 5b 58 48 01 d8 50 5b 58  48 01 d8 50 48 b8 40 08 
0x0000000110: 00 00 00 00 00 00 50 5b  58 48 f7 eb 50 5b 58 48 
0x0000000120: 01 d8 50 5b 58 48 29 d8  50 48 b8 bf f9 ff ff ff 
0x0000000130: ff ff ff 50 5b 58 48 f7  eb 50 5b 58 48 29 d8 50 
0x0000000140: 48 b8 2d f1 ff ff ff ff  ff ff 50 5b 58 48 f7 eb 
0x0000000150: 50 5b 58 48 01 d8 50 48  b8 51 fa ff ff ff ff ff 
0x0000000160: ff 50 5b 58 48 29 d8 50  58 5b 5d c3 
res_jit:         -54609047902875

AST: (((-877 / 1741) / -2556) - ((-3834 - 32) / (977 * (1747 / 301))))
res_interpreter: 0
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 93  fc ff ff ff ff ff ff 50 
0x0000000010: 48 b8 cd 06 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 48 b8 04 f6 ff  ff ff ff ff ff 50 5b 58 
0x0000000030: 48 99 48 f7 fb 50 48 b8  06 f1 ff ff ff ff ff ff 
0x0000000040: 50 48 b8 20 00 00 00 00  00 00 00 50 5b 58 48 29 
0x0000000050: d8 50 48 b8 d1 03 00 00  00 00 00 00 50 48 b8 d3 
0x0000000060: 06 00 00 00 00 00 00 50  48 b8 2d 01 00 00 00 00 
0x0000000070: 00 00 50 5b 58 48 99 48  f7 fb 50 5b 58 48 f7 eb 
0x0000000080: 50 5b 58 48 99 48 f7 fb  50 5b 58 48 29 d8 50 58 
0x0000000090: 5b 5d c3 
res_jit:         0

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (-272 - 3657)
res_interpreter: -3929
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 f0  fe ff ff ff ff ff ff 50 
0x0000000010: 48 b8 49 0e 00 00 00 00  00 00 50 5b 58 48 29 d8 
0x0000000020: 50 58 5b 5d c3 
res_jit:         -3929

AST: (523 + (((-1939 + (-188 + ((((12 / -4019) / (150 + ((1098 / -222) * 3067))) * 1032) * 1682))) / -1585) + -1096))
res_interpreter: -572
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 0b  02 00 00 00 00 00 00 50 
0x0000000010: 48 b8 6d f8 ff ff ff ff  ff ff 50 48 b8 44 ff ff 
0x0000000020: ff ff ff ff ff 50 48 b8  0c 00 00 00 00 00 00 00 
0x0000000030: 50 48 b8 4d f0 ff ff ff  ff ff ff 50 5b 58 48 99 
0x0000000040: 48 f7 fb 50 48 b8 96 00  00 00 00 00 00 00 50 48 
0x0000000050: b8 4a 04 00 00 00 00 00  00 50 48 b8 22 ff ff ff 
0x0000000060: ff ff ff ff 50 5b 58 48  99 48 f7 fb 50 48 b8 fb 
0x0000000070: 0b 00 00 00 00 00 00 50  5b 58 48 f7 eb 50 5b 58 
0x0000000080: 48 01 d8 50 5b 58 48 99  48 f7 fb 50 48 b8 08 04 
0x0000000090: 00 00 00 00 00 00 50 5b  58 48 f7 eb 50 48 b8 92 
0x00000000a0: 06 00 00 00 00 00 00 50  5b 58 48 f7 eb 50 5b 58 
0x00000000b0: 48 01 d8 50 5b 58 48 01  d8 50 48 b8 cf f9 ff ff 
0x00000000c0: ff ff ff ff 50 5b 58 48  99 48 f7 fb 50 48 b8 b8 
0x00000000d0: fb ff ff ff ff ff ff 50  5b 58 48 01 d8 50 5b 58 
0x00000000e0: 48 01 d8 50 58 5b 5d c3 
res_jit:         -572

AST: (2100 * ((2397 - (-2206 * 2592)) - 4010))
res_interpreter: 12004311900
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 34  08 00 00 00 00 00 00 50 
0x0000000010: 48 b8 5d 09 00 00 00 00  00 00 50 48 b8 62 f7 ff 
0x0000000020: ff ff ff ff ff 50 48 b8  20 0a 00 00 00 00 00 00 
0x0000000030: 50 5b 58 48 f7 eb 50 5b  58 48 29 d8 50 48 b8 aa 
0x0000000040: 0f 00 00 00 00 00 00 50  5b 58 48 29 d8 50 5b 58 
0x0000000050: 48 f7 eb 50 58 5b 5d c3 
res_jit:         12004311900

AST: (((577 + 349) - -2668) + ((950 + ((3113 + ((-1431 * (3881 + (-633 + (-527 + ((((-3470 - (-2346 * -2587)) + (-602 / (-3183 + -2908))) * 2449) - 2401))))) * -1234)) - ((404 - (1658 - (-495 / ((-2652 - 1550) / -1791)))) + (-635 - -3388)))) + 3566))
res_interpreter: -26261301272755861
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 41  02 00 00 00 00 00 00 50 
0x0000000010: 48 b8 5d 01 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000020: 50 48 b8 94 f5 ff ff ff  ff ff ff 50 5b 58 48 29 
0x0000000030: d8 50 48 b8 b6 03 00 00  00 00 00 00 50 48 b8 29 
0x0000000040: 0c 00 00 00 00 00 00 50  48 b8 69 fa ff ff ff ff 
0x0000000050: ff ff 50 48 b8 29 0f 00  00 00 00 00 00 50 48 b8 
0x0000000060: 87 fd ff ff ff ff ff ff  50 48 b8 f1 fd ff ff ff 
0x0000000070: ff ff ff 50 48 b8 72 f2  ff ff ff ff ff ff 50 48 
0x0000000080: b8 d6 f6 ff ff ff ff ff  ff 50 48 b8 e5 f5 ff ff 
0x0000000090: ff ff ff ff 50 5b 58 48  f7 eb 50 5b 58 48 29 d8 
0x00000000a0: 50 48 b8 a6 fd ff ff ff  ff ff ff 50 48 b8 91 f3 
0x00000000b0: ff ff ff ff ff ff 50 48  b8 a4 f4 ff ff ff ff ff 
0x00000000c0: ff 50 5b 58 48 01 d8 50  5b 58 48 99 48 f7 fb 50 
0x00000000d0: 5b 58 48 01 d8 50 48 b8  91 09 00 00 00 00 00 00 
0x00000000e0: 50 5b 58 48 f7 eb 50 48  b8 61 09 00 00 00 00 00 
0x00000000f0: 00 50 5b 58 48 29 d8 50  5b 58 48 01 d8 50 5b 58 
0x0000000100: 48 01 d8 50 5b 58 48 01  d8 50 5b 58 48 f7 eb 50 
0x0000000110: 48 b8 2e fb ff ff ff ff  ff ff 50 5b 58 48 f7 eb 
0x0000000120: 50 5b 58 48 01 d8 50 48  b8 94 01 00 00 00 00 00 
0x0000000130: 00 50 48 b8 7a 06 00 00  00 00 00 00 50 48 b8 11 
0x0000000140: fe ff ff ff ff ff ff 50  48 b8 a4 f5 ff ff ff ff 
0x0000000150: ff ff 50 48 b8 0e 06 00  00 00 00 00 00 50 5b 58 
0x0000000160: 48 29 d8 50 48 b8 01 f9  ff ff ff ff ff ff 50 5b 
0x0000000170: 58 48 99 48 f7 fb 50 5b  58 48 99 48 f7 fb 50 5b 
0x0000000180: 58 48 29 d8 50 5b 58 48  29 d8 50 48 b8 85 fd ff 
0x0000000190: ff ff ff ff ff 50 48 b8  c4 f2 ff ff ff ff ff ff 
0x00000001a0: 50 5b 58 48 29 d8 50 5b  58 48 01 d8 50 5b 58 48 
0x00000001b0: 29 d8 50 5b 58 48 01 d8  50 48 b8 ee 0d 00 00 00 
0x00000001c0: 00 00 00 50 5b 58 48 01  d8 50 5b 58 48 01 d8 50 
0x00000001d0: 58 5b 5d c3 
res_jit:         -26261301272755861

AST: (((-611 + ((-4010 - ((-2098 * -2237) / 728)) + -648)) - -626) - 2979)
res_interpreter: -14068
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 9d  fd ff ff ff ff ff ff 50 
0x0000000010: 48 b8 56 f0 ff ff ff ff  ff ff 50 48 b8 ce f7 ff 
0x0000000020: ff ff ff ff ff 50 48 b8  43 f7 ff ff ff ff ff ff 
0x0000000030: 50 5b 58 48 f7 eb 50 48  b8 d8 02 00 00 00 00 00 
0x0000000040: 00 50 5b 58 48 99 48 f7  fb 50 5b 58 48 29 d8 50 
0x0000000050: 48 b8 78 fd ff ff ff ff  ff ff 50 5b 58 48 01 d8 
0x0000000060: 50 5b 58 48 01 d8 50 48  b8 8e fd ff ff ff ff ff 
0x0000000070: ff 50 5b 58 48 29 d8 50  48 b8 a3 0b 00 00 00 00 
0x0000000080: 00 00 50 5b 58 48 29 d8  50 58 5b 5d c3 
res_jit:         -14068

AST: (-13 / (-857 * -2633))
res_interpreter: 0
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 f3  ff ff ff ff ff ff ff 50 
0x0000000010: 48 b8 a7 fc ff ff ff ff  ff ff 50 48 b8 b7 f5 ff 
0x0000000020: ff ff ff ff ff 50 5b 58  48 f7 eb 50 5b 58 48 99 
0x0000000030: 48 f7 fb 50 58 5b 5d c3 
res_jit:         0

AST: ((-2243 - 289) * ((3882 * (3447 + 3806)) * (((-2395 + 1675) + 1201) - 3589)))
res_interpreter: 221573552076576
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 3d  f7 ff ff ff ff ff ff 50 
0x0000000010: 48 b8 21 01 00 00 00 00  00 00 50 5b 58 48 29 d8 
0x0000000020: 50 48 b8 2a 0f 00 00 00  00 00 00 50 48 b8 77 0d 
0x0000000030: 00 00 00 00 00 00 50 48  b8 de 0e 00 00 00 00 00 
0x0000000040: 00 50 5b 58 48 01 d8 50  5b 58 48 f7 eb 50 48 b8 
0x0000000050: a5 f6 ff ff ff ff ff ff  50 48 b8 8b 06 00 00 00 
0x0000000060: 00 00 00 50 5b 58 48 01  d8 50 48 b8 b1 04 00 00 
0x0000000070: 00 00 00 00 50 5b 58 48  01 d8 50 48 b8 05 0e 00 
0x0000000080: 00 00 00 00 00 50 5b 58  48 29 d8 50 5b 58 48 f7 
0x0000000090: eb 50 5b 58 48 f7 eb 50  58 5b 5d c3 
res_jit:         221573552076576

AST: (((-957 + -3691) - ((-326 * 1857) - 1934)) / (84 + ((-1063 - 1476) - 3831)))
res_interpreter: -95
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 43  fc ff ff ff ff ff ff 50 
0x0000000010: 48 b8 95 f1 ff ff ff ff  ff ff 50 5b 58 48 01 d8 
0x0000000020: 50 48 b8 ba fe ff ff ff  ff ff ff 50 48 b8 41 07 
0x0000000030: 00 00 00 00 00 00 50 5b  58 48 f7 eb 50 48 b8 8e 
0x0000000040: 07 00 00 00 00 00 00 50  5b 58 48 29 d8 50 5b 58 
0x0000000050: 48 29 d8 50 48 b8 54 00  00 00 00 00 00 00 50 48 
0x0000000060: b8 d9 fb ff ff ff ff ff  ff 50 48 b8 c4 05 00 00 
0x0000000070: 00 00 00 00 50 5b 58 48  29 d8 50 48 b8 f7 0e 00 
0x0000000080: 00 00 00 00 00 50 5b 58  48 29 d8 50 5b 58 48 01 
0x0000000090: d8 50 5b 58 48 99 48 f7  fb 50 58 5b 5d c3 
res_jit:         -95

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: (2043 * -4031)
res_interpreter: -8235333
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 fb  07 00 00 00 00 00 00 50 
0x0000000010: 48 b8 41 f0 ff ff ff ff  ff ff 50 5b 58 48 f7 eb 
0x0000000020: 50 58 5b 5d c3 
res_jit:         -8235333

AST: (575 + 1497)
res_interpreter: 2072
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 3f  02 00 00 00 00 00 00 50 
0x0000000010: 48 b8 d9 05 00 00 00 00  00 00 50 5b 58 48 01 d8 
0x0000000020: 50 58 5b 5d c3 
res_jit:         2072

AST: ((2272 / 755) + -1235)
res_interpreter: -1232
JIT CODE:
0x0000000000: 55 48 89 e5 53 48 b8 e0  08 00 00 00 00 00 00 50 
0x0000000010: 48 b8 f3 02 00 00 00 00  00 00 50 5b 58 48 99 48 
0x0000000020: f7 fb 50 48 b8 2d fb ff  ff ff ff ff ff 50 5b 58 
0x0000000030: 48 01 d8 50 58 5b 5d c3 
res_jit:         -1232

test tests::test_compare_interpreter_jit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.97s
```
