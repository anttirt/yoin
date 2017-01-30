## Yoin - A Japanese Morphological Analyzer

**This is still under development...**

[![Build Status](https://travis-ci.org/agatan/yoin.svg?branch=master)](https://travis-ci.org/agatan/yoin)

`yoin` is a Japanese morphological analyze engine written in pure Rust.

[mecab-ipadic](https://taku910.github.io/mecab/) is embedded in `yoin`.

```sh
:) $ yoin
すもももももももものうち
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
```

## Build & Install

```sh
:) $ git clone https://github.com/agatan/yoin
:) $ cd yoin && cargo install
```

You can execute `yoin` command, or use `yoin` as a library.

## Usage - CLI

Currently, `yoin` has no option...
`yoin` just reads lines from stdin, analyzes each line, and outputs results.

```sh
:) $ yoin
すもももももももものうち
すもも	名詞,一般,*,*,*,*,すもも,スモモ,スモモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
も	助詞,係助詞,*,*,*,*,も,モ,モ
もも	名詞,一般,*,*,*,*,もも,モモ,モモ
の	助詞,連体化,*,*,*,*,の,ノ,ノ
うち	名詞,非自立,副詞可能,*,*,*,うち,ウチ,ウチ
EOS
そこではなしは終わりになった
そこで	接続詞,*,*,*,*,*,そこで,ソコデ,ソコデ
はなし	名詞,一般,*,*,*,*,はなし,ハナシ,ハナシ
は	助詞,係助詞,*,*,*,*,は,ハ,ワ
終わり	動詞,自立,*,*,五段・ラ行,連用形,終わる,オワリ,オワリ
に	助詞,格助詞,一般,*,*,*,に,ニ,ニ
なっ	動詞,自立,*,*,五段・ラ行,連用タ接続,なる,ナッ,ナッ
た	助動詞,*,*,*,特殊・タ,基本形,た,タ,タ
EOS
```

## LICENSE

This software in under the MIT License and contains the MeCab-ipadic model.
See `LICENSE` and `NOTICE.txt` for more details.
