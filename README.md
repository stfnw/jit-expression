This is a small demo project that implements a simple JIT for evaluating arithmetic expressions.
It assumes it is running under Linux on x86_64.
It uses 64 bit signed numbers and supports integer addition, subtraction, multiplication and division.

# How to build and run

Provide the expression to be evaluated on the commandline like so:

```
$ cargo run "2+3 *4 + 10/5 - 6*(9+7)"
   Compiling jit-expression-eval v0.1.0 (jit-expression)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/jit-expression-eval '2+3 *4 + 10/5 - 6*(9+7)'`
AST: (((2 + (3 * 4)) + (10 / 5)) - (6 * (9 + 7)))
res_interpreter: -80
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x02\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x03\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x04\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x0a\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x05\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x06\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x09\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x07\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         -80
```

# How to test

The included unit tests randomly generate some arithmetic expressions,
compute the result both by interpreting the resulting syntax tree and by jitting it down to native machine code and running that,
and assert that both results agree with each other.
One example run:

```
$ cargo test -- --nocapture
   Compiling jit-expression-eval v0.1.0 (/home/self/sync-dev/jit-expression-eval)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests src/main.rs (target/debug/deps/jit_expression_eval-13417cf070e179bf)

running 1 test
AST: ((-603 / (2908 + (-3508 * -376))) * 2828)
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xa5\xfd\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x5c\x0b\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x4c\xf2\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x88\xfe\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x0c\x0b\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x58\x5b\x5d\xc3
res_jit:         0

AST: (-1978 - 988)
res_interpreter: -2966
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x46\xf8\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xdc\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         -2966

AST: ((((-3441 + ((638 - (1690 + (3610 + (-3378 * ((790 / (-2077 / -890)) * (-6 - (2625 * (2890 - 606)))))))) / -2060)) - 2662) * 279) + 2002)
res_interpreter: 1083474973246
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x8f\xf2\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x7e\x02\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x9a\x06\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x1a\x0e\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xce\xf2\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x16\x03\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xe3\xf7\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x86\xfc\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xfa\xff\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x41\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x4a\x0b\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x5e\x02\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xf4\xf7\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x66\x0a\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x17\x01\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xd2\x07\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         1083474973246

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: ((1894 * -4051) * ((-3669 / (-2777 + -2902)) / (315 + -1274)))
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x66\x07\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x2d\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xab\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x27\xf5\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xaa\xf4\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x3b\x01\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x06\xfb\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\xf7\xeb\x50\x58\x5b\x5d\xc3
res_jit:         0

AST: (-918 - 2979)
res_interpreter: -3897
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x6a\xfc\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xa3\x0b\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         -3897

AST: (((616 / (-2573 * (-2468 * (-1255 * -1714)))) + 3788) + -1394)
res_interpreter: 2394
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x68\x02\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xf3\xf5\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x5c\xf6\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x19\xfb\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x4e\xf9\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xcc\x0e\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x8e\xfa\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         2394

AST: (-958 + -1892)
res_interpreter: -2850
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x42\xfc\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x9c\xf8\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         -2850

AST: (((2933 / (((((1015 + -1904) * -2464) / 1099) - -1503) - -2453)) - -160) - -1653)
res_interpreter: 1813
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x75\x0b\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xf7\x03\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x90\xf8\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x60\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x4b\x04\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x21\xfa\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x6b\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x60\xff\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x8b\xf9\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         1813

AST: (((1959 - -95) + 1011) + 3447)
res_interpreter: 6512
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xa7\x07\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xa1\xff\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xf3\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x77\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         6512

AST: ((-2197 + 4055) / 3377)
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x6b\xf7\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xd7\x0f\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x31\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x58\x5b\x5d\xc3
res_jit:         0

AST: ((-3162 + 601) / (-3705 / ((-765 / -2029) - (((2271 / (-2080 + (1892 / 1682))) - 2474) / 1430))))
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xa6\xf3\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x59\x02\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x87\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x03\xfd\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x13\xf8\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xdf\x08\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xe0\xf7\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x64\x07\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x92\x06\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xaa\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x96\x05\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x58\x5b\x5d\xc3
res_jit:         0

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: ((((-2422 / -3967) * (-674 - -1444)) * ((((248 * -2171) - (2573 - (2719 + ((-3099 + ((-3711 / 3987) / (-92 + -209))) - -1799)))) + (-1935 - (2582 - -2505))) - (-649 + (-3812 * -1835)))) * 2363)
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x8a\xf6\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x81\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x5e\xfd\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x5c\xfa\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xf8\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x85\xf7\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x0d\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x9f\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xe5\xf3\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x81\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x93\x0f\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xa4\xff\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x2f\xff\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xf9\xf8\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x71\xf8\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x16\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x37\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x77\xfd\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x1c\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xd5\xf8\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x3b\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x58\x5b\x5d\xc3
res_jit:         0

AST: ((((((1443 * (-2799 - (((3181 + -2455) - 2366) * (((724 / (874 + 181)) * 437) + ((((1694 - -3037) + -4068) * ((-1912 + -3810) - (1718 - ((-2988 / 206) - 1289)))) - (452 * (((((-1912 / -742) * (3974 - 2083)) / ((((-431 - 1459) / ((928 + 939) - 3407)) * (-3608 * 3135)) + 3935)) * 855) - 1957))))))) + (-3259 * ((-3789 + 1016) + 3478))) / 1521) - 2599) * -2251) + ((((-299 - -570) - -2325) + 3872) + -2664))
res_interpreter: 17203593865039
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xa3\x05\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x11\xf5\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x6d\x0c\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x69\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x3e\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xd4\x02\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x6a\x03\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xb5\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xb5\x01\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x9e\x06\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x23\xf4\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x1c\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x88\xf8\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x1e\xf1\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xb6\x06\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x54\xf4\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xce\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x09\x05\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xc4\x01\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x88\xf8\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x1a\xfd\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x86\x0f\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x23\x08\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x51\xfe\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xb3\x05\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xa0\x03\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xab\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x4f\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xe8\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x3f\x0c\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x5f\x0f\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x57\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xa5\x07\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x45\xf3\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x33\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xf8\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x96\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xf1\x05\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x27\x0a\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x35\xf7\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xd5\xfe\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xc6\xfd\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xeb\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x20\x0f\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x98\xf5\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         17203593865039

AST: ((((2043 - (((((3556 * 3839) / 2439) / ((-2627 - (117 + (((654 + -3842) * (((1706 * ((-3229 - 3931) + 3661)) + -4026) + -3933)) - 2325))) * -1045)) - ((2775 - ((3393 - 3249) * ((850 + (3829 - (((-377 / 1946) / 865) - 2429))) - 632))) / ((-3609 * 674) * ((-2303 - 3526) + 4)))) + -707)) + 2464) / -2195) / 1645)
res_interpreter: 0
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xfb\x07\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xe4\x0d\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xff\x0e\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x87\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xbd\xf5\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x75\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x8e\x02\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xfe\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xaa\x06\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x63\xf3\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x5b\x0f\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x4d\x0e\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x46\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xa3\xf0\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x15\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xeb\xfb\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xd7\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x41\x0d\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xb1\x0c\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x52\x03\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xf5\x0e\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x87\xfe\xff\xff\xff\xff\xff\xff\x50\x48\xb8\x9a\x07\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x61\x03\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x7d\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x78\x02\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xe7\xf1\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xa2\x02\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x01\xf7\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xc6\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x04\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x3d\xfd\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xa0\x09\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x6d\xf7\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\x6d\x06\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x58\x5b\x5d\xc3
res_jit:         0

AST: (((3910 + 3562) * (2810 / (2747 - 1900))) - (120 + (-3854 / -2319)))
res_interpreter: 22295
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x46\x0f\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xea\x0d\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\xfa\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xbb\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x6c\x07\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\x78\x00\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xf2\xf0\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xf1\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         22295

Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!

AST: ((((4022 * -1085) / -2313) - 1225) + (3635 * ((661 - 3759) + (-1332 - 221))))
res_interpreter: -16905724
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xb6\x0f\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xc3\xfb\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\xf7\xeb\x50\x48\xb8\xf7\xf6\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x48\xb8\xc9\x04\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\x33\x0e\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x95\x02\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xaf\x0e\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x48\xb8\xcc\xfa\xff\xff\xff\xff\xff\xff\x50\x48\xb8\xdd\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x01\xd8\x50\x5b\x58\x48\xf7\xeb\x50\x5b\x58\x48\x01\xd8\x50\x58\x5b\x5d\xc3
res_jit:         -16905724

AST: (3929 * 220)
res_interpreter: 864380
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x59\x0f\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xdc\x00\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\xf7\xeb\x50\x58\x5b\x5d\xc3
res_jit:         864380

AST: (2397 / (2757 - 3103))
res_interpreter: -6
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\x5d\x09\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xc5\x0a\x00\x00\x00\x00\x00\x00\x50\x48\xb8\x1f\x0c\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x29\xd8\x50\x5b\x58\x48\x99\x48\xf7\xfb\x50\x58\x5b\x5d\xc3
res_jit:         -6

AST: ((1200 + 3780) - -2989)
res_interpreter: 7969
JIT CODE: \x55\x48\x89\xe5\x53\x48\xb8\xb0\x04\x00\x00\x00\x00\x00\x00\x50\x48\xb8\xc4\x0e\x00\x00\x00\x00\x00\x00\x50\x5b\x58\x48\x01\xd8\x50\x48\xb8\x53\xf4\xff\xff\xff\xff\xff\xff\x50\x5b\x58\x48\x29\xd8\x50\x58\x5b\x5d\xc3
res_jit:         7969

test tests::test_compare_interpreter_jit ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

[Finished running. Exit status: 0]
```
