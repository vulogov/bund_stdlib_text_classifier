# Module for BUND standard library: Naive Bayes text classifier

A powerful and flexible text classification module built on top of the bund machine learning framework. This module is part of the
Bund Standard Library (stdlib) and provides a streamlined workflow for training, evaluating, and deploying text classification models.
This module is a library and not designed to be a standalone application, but it can be embedded inside BUND virtual machine.

## Installation

This module required _make_ and _Rust_ framework to be installed first. After that:

```bash
cargo add bund_stdlib_text_classifier
```

## Quick start

Get started with a simple example to classify text data. First, you have to create and train classifier

```rust
:TEST textclassifier.new
    :rust "./scripts/rust.txt" textclassifier.train.from_file
    "{A} tokens for RUST" format println
    :kant "./scripts/kant.txt" textclassifier.train.from_file
    "{A} tokens for KANT" format println
    :astronomy "./scripts/astronomy.txt" textclassifier.train.from_file
    "{A} tokens for ASTRONOMY" format println
    :tolstoy "./scripts/tolstoy.txt" textclassifier.train.from_file
    "{A} tokens for LEO TOLSTOY" format println
    textclassifier.train.finish
```

Then you can classify any text lines.

```rust
:TEST
  "At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code"
    textclassifier.classify
```

The following call will return a DICT value:

```json
{
  "astronomy": 0.8331765363980779,
  "kant": 0.9968812285706273,
  "rust": 1.0,
  "tolstoy": 0.9968812285706273
}
```
## BUND functions exposed in this module

| Name | Stack IN | Stack OUT | Description |
|===========================================|
| textclassifier.new | - Classifier name | - Classifier name | Create new classifier |
|===========================================|
