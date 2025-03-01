# MartelDB

## Introduction

MartelDB is a personal project to learn about database implementation and Rust. This is a toy project where the primary goal is learning. 
With this goal in mind, there is a desire to minimize external dependencies to the maximum extent possible.

### Project scope

The ideal scope of the project is a fully functioning relational DBMS working with a subset of SQL. The current plans include support for 
transactions, and concurrent readers/writers. The grammar of SQL supported matches most closely to sqlite, but several statements more closely 
follow postgres. A full list of grammar supported can be seen in the sqlgrammar file.

## Current Progress

The project is still in its beginning stages. Currently work is contained to the SQL parser and AST generation.

## Parser

The parser is a hand-written recursive descent parser. This was chosen as opposed to the numerous parser generator solutions for a couple reasons. 
The first being that this is a learning project. While writing a recursive descent parser is not necessarily new to me, I had not yet done it in Rust 
or for SQL. The second reason would be for parse error handling. I found many parser generators in rust did not provide adequate input error handling that I 
hope to support in this project. 
