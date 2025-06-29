# BPE Tokenizer

A simple Byte Pair Encoding (BPE) tokenizer implementation in Rust.

## Usage

```console
> cargo run -r -- -h
Usage: target/release/bpe -n <vocabulary_size> <filepath>

Performs Byte Pair Encoding (BPE) tokenization on a text file.

Arguments:
  <filepath>              Path to the text file to tokenize

Options:
  -n <vocabulary_size>    Number of token pairs to learn

Examples:
  target/release/bpe -n 1000 input.txt
```

```console
> cargo run -r -- ceara_wikipedia.txt -n 10000
[o ceara e ][uma das ][27 ][unidades federativ][as do brasil. ][esta ][situ][ado ][no ][norte ][da regiao ][nordeste, ][faz ][divi][sa ][com ][rio grande do nor][te e ][paraib][a a ][leste, pernambuco ][ao sul][ e ][piaui ][a oeste, ][alem de ser banha][do pelo ][oceano atlantico ][a ][nor][te e ][nordeste. sua ][area ][total][ e de ][148 ][894,44][2 ][km2,[2] ocupan][do 9,3][7][% da ][area do ][nordes][te e ][1,][74][% da ][super][fici][e ][do brasil. ][de acordo com o censo do ibge, ][a populacao ][do estado em 20][2][2 ][era de ][8][.79][4][.][95][7 habitan][tes, ][sendo o ][oita][vo ][estado mais populoso do pai][s][.[2]
```

