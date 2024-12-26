# Voxlang - Another Simple Programming Language

Welcome to **VoxLang**, a programming language built as a learning project for compiler development. This repository serves as the official home for VoxLang, documenting its features, syntax, and the progress of its development.

## Part One: A Basic Parser

The most fundamental part of any language is the parser. It is a piece of software whose purpose is to take a flat structure (usually text in some form) and convert it into a tree structure.

First, we will make a very simple parser that can parse mathematical expression that do not contain nesting.

- `1+1` is allowed
- `2 * 4 + 5` is not allowed since it is (2 \* 4) + 5

The basic expression contains an LHS, an operator and an RHS. Where LHS and RHS are numbers and the operator is any one of, '+', '-', '\*' and '/'.

To parse the expression, We take the expression as a string, and then take the first digit as LHS and send the remainder of the string to extract the operator. After extracting the operator (length=1), we send the remainder to extract the RHS.

## Part Two: Whitespace Support