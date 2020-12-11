# bf-derivatives-tool

let's create your own brainfuck derivative.

## Setup

```bash
$ git clone https://github.com/guricerin/bf-derivatives-tool
$ cd bf-derivatives-tool/
$ cargo install --path .
```

After installation, two commands will be available.

* bfi
* bft

## Usage

### Edit the json file and write grammar rules

* the following example is quoted from http://kmaebashi.com/zakki/lang0003.html

```bash
$ vim jojo-grammar.json
```

```json
{
    "rshift": "ふるえるぞハート!",
    "lshift": "燃えつきるほどヒート!!",
    "inc": "オラ",
    "dec": "無駄",
    "write": "ァ!",
    "read": "やれやれだぜ",
    "loop_begin": "おまえの次のセリフは「",
    "loop_end": "」という!"
}
```

Grammar json file format: 

| JSON item | brainfuck command |
|:---------:|:-----------------:|
| rshift | > |
| lshift | < |
| inc | + |
| dec | - |
| write | . |
| read | , |
| loop_begin | [ |
| loop_end | ] |

### Run the program

#### Original brainfuck

```bash
$ bfi /path/to/brainfuck/code/file
```

#### Your brainfuck derivative

```bash
$ vim hello.jojo
```

```
ふるえるぞハート!
オラオラオラオラオラオラオラオラオラオラオラオラオラオラオラオラ
おまえの次のセリフは「燃えつきるほどヒート!!オラオラオラオラオラオラ
ふるえるぞハート!無駄」という!
燃えつきるほどヒート!!オラオラオラオラオラオラオラオラァ!
無駄無駄無駄ァ!
オラオラオラオラオラオラオラァ!ァ!
オラオラオラァ!ふるえるぞハート!
オラオラオラオラオラオラオラオラオラオラオラオラオラオラオラオラ
おまえの次のセリフは「ふるえるぞハート!オラオラ燃えつきるほどヒート!!無駄」という!
ふるえるぞハート!オラオラオラオラオラオラオラオラオラオラオラオラァ!
無駄無駄無駄無駄無駄無駄無駄無駄無駄無駄無駄無駄ァ!
燃えつきるほどヒート!!燃えつきるほどヒート!!オラオラオラオラオラオラオラオラァ!
無駄無駄無駄無駄無駄無駄無駄無駄ァ!
オラオラオラァ!
無駄無駄無駄無駄無駄無駄ァ!
無駄無駄無駄無駄無駄無駄無駄無駄ァ!
ふるえるぞハート!
ふるえるぞハート!
オラオラオラオラオラオラオラオラオラオラオラオラオラオラァ!
```

```bash
$ bfi /path/to/hello.jojo -g /path/to/jojo-grammar.json
```

Output:

```bash
hello, world.
```

### Translate bf code -> bf-derative code

```bash
$ bft /path/to/bf-code-file/ -t /path/to/bf-derative-grammar.json
```

### Translate bf-derative code -> bf code

```bash
$ bft /path/to/bf-derative-code-file/ -f /path/to/bf-derative-grammar.json
```
