# brainfuck-maker

let's make your own brainfuck derivative.

## Usage

### original brainfuck

```bash
$ cargo run -- -c /path/to/brainfuck/code/file
```

### your brainfuck derivative

1. edit the json file and write grammar rules.

- quote from http://kmaebashi.com/zakki/lang0003.html

```json
{
    "rshift": "ふるえるぞハート!", // >
    "lshift": "燃えつきるほどヒート!!", // <
    "inc": "オラ", // +
    "dec": "無駄", // -
    "write": "ァ!", // .
    "read": "やれやれだぜ", // ,
    "loop_begin": "おまえの次のセリフは「", // [
    "loop_end": "」という!" // ]
}
```

1. write the brainfuck derivative code.

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

1. run.

```bash
$ cargo run -- -c /path/to/brainfuck/code/file -g /path/to/grammar/json/file
```