# rmonkey
An interpreter for [monkey programing language](https://monkeylang.org/) written in Rust

## What's monkey
- C-like syntax
- variable bindings
- integer, boolean and string
- basic data structure(array, hashmap)
- arithmetic expression (+ - * /)
- build-in function
- first-class and high-order functions
- closures

### Data Types
- Integer
- String
- Boolean
- Array
- HashMap

### Build-in Functions

`len(<arg>): Integer`
```
len("hello") // 5
len([1, 2, 3]) // 3
```

`first(<arg>): Object`
```
first([1, 2, 3]); // 1
```

`last(<arg>): Object`
```
last([1, 2, 3]) // 3
```

`rest(<arg>): Array`
```
rest([1, 2, 3]) // [2, 3]
```

`push(<arg1>, <arg2>): Array`
```
push([1, 2], 3) // [1, 2, 3]
```