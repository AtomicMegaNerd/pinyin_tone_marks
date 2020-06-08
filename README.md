# Rust Pinyin Tone Mark Converter

## Introduction

This Rust program will take a source text file with numerical Pinyin markup (Ni3
hao3) and it will convert it to use the proper Pinyin tone marks (Nǐ Hǎo).

The rules for writing the source documents are very simple: For the pinyin
corresponding to each character put a number from 1-4 following it for the tone.
Ignore the neutral tone. Use v for ü.

## Source Pinyin Examples

* 你好(nǐhǎo) you would write: ni3hao3.
* 现在让我们都考虑一下(Xiànzài ràng wǒmen dōu kǎolǜ yíxià) write: Xian4zai4 rang4 wo3men
dou1 kao3lv4 yi2xia4 (not the v used for ü).

## Acknowledgements

This is inspired by the Pinyin Joe macros for Word and Excel, though I totally designed
this implementation myself and did not even examine the source code for those macros
as this was a programming challenge I wanted to do for fun:

[https://www.pinyinjoe.com/pinyin/pinyin_macro_faq.htm](https://www.pinyinjoe.com/pinyin/pinyin_macro_faq.htm)
