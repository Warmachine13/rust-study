# to install cargo libraries

```sh
cargo install cargo watch
```

to use this library

```sh
cargo watch -x run
```

will watch changes in code



variables rust inside function 

```rs
let a = 5;
```

ignore variable by debuger

```rs
let _a = 5;
```

inferency of types

```rs
let total: i32 = 30;
```

mutable variables for change value

```rs
let mut total = 30;
total = 40;
```
Variable shadowing is use for scope 

```rs
let total = 30;
let total = "quarenta";
```

separeted scope is possible by using {} 

```rs
let total = 30
{
    let total = 40
}
```
and you can use variables from another scope 

```rs
let total = 30;
{
    let total = total + 20;
}
```

constants can be use out functions and inside fuctions and by conversion use Upper snake case and need type of variable and contants cant't be defined two times
```rs
const TOTAL: u32 = 30;
```
can be used for make operations when can be defined on operation

```rs
const TOTAL: u32 = 30;
const FULL_TOTAL: u32 = TOTAL + 1;
```

in rust variables need be defined in snake case and lower case let

```rs
const total_in_hours = 10
```