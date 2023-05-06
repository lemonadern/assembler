# Assembler

Assembler for nktk-architecture computer.

## How to use

### Assembling

1. Clone this repository.
2. Execute `cargo run <input_file>`

```
git clone https://github.com/lemonadern/assembler.git
cargo r input.asm
```

### help command

By executing `cargo run -- --help`, you can see the usage instructions for the
command.

```
Usage: assembler [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Input file path

Options:
  -b, --base-address <BASE_ADDRESS>  Address for first instruction [default: 0]
  -o, --output <FILE>                Output file [default: output.txt]
  -h, --help                         Print help
  -V, --version                      Print version
```

## Instructions

| Numbering | Name | Assembly format    | Status |
| --------- | ---- | ------------------ | ------ |
| 0         | add  | `add rd, rs, rt`   | ✅     |
| 1         | addi | `addi rt, rs, imm` | ✅     |
| 2         | lw   | `lw rt, addr(rs)`  | ✅     |
| 3         | sw   | `sw rt, addr(rs)`  | ✅     |
| 4         | beq  | `beq rs, rt, addr` | ✅     |
| 5         | j    | `j addr`           | ✅     |
| 6         | jal  | `jal addr`         | ✅     |
| 7         | jr   | `jr rs`            | ✅     |

## レジスタ

| Register Name | Status |
| ------------- | ------ |
| `$0`, `$zero` | ✅     |
| `$1`, `$at`   | ✅     |
| `$2`, `$v0`   | ✅     |
| `$3`, `$v1`   | ✅     |
| `$4`, `$a0`   | ✅     |
| `$5`, `$a1`   | ✅     |
| `$6`, `$a2`   | ✅     |
| `$7`, `$a3`   | ✅     |
| `$8`, `$t0`   | ✅     |
| `$9`, `$t1`   | ✅     |
| `$10`, `$t2`  | ✅     |
| `$11`, `$t3`  | ✅     |
| `$12`, `$t4`  | ✅     |
| `$13`, `$t5`  | ✅     |
| `$14`, `$t6`  | ✅     |
| `$15`, `$t7`  | ✅     |
| `$16`, `$s0`  | ✅     |
| `$17`, `$s1`  | ✅     |
| `$18`, `$s2`  | ✅     |
| `$19`, `$s3`  | ✅     |
| `$20`, `$s4`  | ✅     |
| `$21`, `$s5`  | ✅     |
| `$22`, `$s6`  | ✅     |
| `$23`, `$s7`  | ✅     |
| `$24`, `$t8`  | ✅     |
| `$25`, `$t9`  | ✅     |
| `$26`, `$k0`  | ✅     |
| `$27`, `$k1`  | ✅     |
| `$28`, `$gp`  | ✅     |
| `$29`, `$sp`  | ✅     |
| `$30`, `$fp`  | ✅     |
| `$31`, `$ra`  | ✅     |

## アセンブリ言語について

このアセンブラが解釈するアセンブリ言語は、（一般的なアセンブリ言語に比べて）限られた機能しか持ちません。また、パースの作業を簡略なものにするために、非常に限定された文法のみをサポートしています。

### 機能

- 命令（オペコードとオペランド）およびそれに対するラベルの解釈しかサポートしていません。

- ディレクティブはサポート外なので、以下のことはできません
  - データセグメントでのデータ定義
  - データセグメントで定義されたデータに対するラベルを使った値の読み込み
  - 他ファイルと組み合わせての実行

### 文法

BNF書こうと思ったんですが面倒なのでやめます。説明から感じ取って理解してください

- 1行につき1命令
- ラベルを書く場合は行頭に書いて、命令との間に `:` を置く
- `#`による一行コメントが書ける
- コメントのみの行も可
- 空行も可

以下に例を示します：

```assembly
label:  addi    $sp,    $sp,    -4      
        sw      $ra,    0($sp)          # comment
```

### 注意点

その他の制約など

- `beq`, `j`,`jal` 命令におけるオペランド`addr`
  は、ラベル指定による入力のみを受け付けます
