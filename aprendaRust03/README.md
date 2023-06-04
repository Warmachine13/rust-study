# Escalares (scalar types)
- Representam um único valor contido
dentro de uma escala conhecida.
- Permitem a comparação direta entre valores.

## Tipos
- inteiro (integer) ex: '5'
- flutuante (floating point) ex: 42.1 
- booleano (bool) ex: `true`, `false`
- caractere (char) ex: 'a'', 咽,

# Compostos (Compound Types)
Servem para agregar multiplos valores.

## Tipos
- Tupla (tuple) ex: (5, true, 42.1, 'a')`
- Matriz (array) ex: [1, 2, 3, 4, 5, 6]


# Inteiros
| bits | signed | unsigned |
|------|--------|----------|
| 8    |i8      |u8        |
| 16   |i16     |u16       |
| 32   |i32     |u32       |
| 64   |i64     |u64       |
| 128  |i128    |u128      |
| arch |isize    |usize    |

## signed
range: -(2-1) até 2n-1-1
i8: -128 até 127 [-(2 7) até 2 7 - 1]

## unsigned
range: 0 até 2n - 1
u8: 0 até 255 [0 até 2 8 -1]


image.png


# underline on numbers 

```rs
let x = 199_456;
```

will use normative


defining variables 

```rs

fn main() {
    let x = 5;
    let y = 199_456;
    let h = 0xff;
    let o = 0o77;
    let b = 0b1111_0000;
    let by = b'A';
    
}

```


floating numbers 

```rs
fn main() {
    let x: f64 = 42.1;
}

```


char chacters

only use a sigle '

```rs
    let a = 'a';
```


tuples colection of primetive types

```rs
    // tupla
    let numbers: (i32, i32, i32) = (1, 2, 3);
    println!("{:?}", numbers) 
```


the tuple must be fixed number of itens

to access item inside tuple is 

```rs
fn main() {
    // tupla
    let numbers = (1, 2, 3.5);
    println!("{:?}", numbers.0)
}
```

will print 1


the tuples can use destructuring

```rs
fn main() {
    // tupla
    let numbers = (1, 2, 3.5);
    let (a, b, c) = numbers;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
```

will print 
1
2
3.5


you can alter values of tuple with mut

```rs
fn main() {
    // tupla
    let mut numbers = (1, 2, 3.5);
    numbers.0 = 50;

    println!("{:?}", numbers);
}

```

will print (50, 2, 3.5)


you cannot alter the types of itens of tuple 

```rs
fn main() {
    // tupla
    let mut numbers = (1, 2, 3.5);
    numbers.0 = false;

    println!("{:?}", numbers.);
}

```

will trow error[E0308]: mismatched types


you can alter all itens of tuple with

```rs
fn main() {
    // tupla
    let mut numbers = (1, 2, 3);
    numbers = (4, 5, 6);

    println!("{:?}", numbers);
}

```

# ARRAYS

arrays can't use multiple types

```rs
fn main() {
    // array
    let mut numbers = [1, 2, 3, false];

    println!("{:?}", numbers);
}

```

will trow

  |
3 | ... 2, 3, false];
  |           ^^^^^ expected integer, found `bool`


  to access values 

```rs
fn main() {
    // array
    let numbers: [i32; 2] = [1, 2];

    println!("{:?}", numbers[0]);
}

```

will print 1


out of bounds

```rs
fn main() {
    // array
    let numbers: [i32; 2] = [1, 2];

    println!("{:?}", numbers[10]);
}

```

will trow error  index out of bounds: the length is 2 but the index is 10


#Slice

```rs
fn main() {
    // array
    let numbers = [1, 2, 3];

    println!("{:?}", &numbers[1..2]);
}

```

will show [2] and if

```rs
fn main() {
    // array
    let numbers = [1, 2, 3];

    println!("{:?}", &numbers[1..2]);
}

```
will show [2,3] and 


```rs
fn main() {
    // array
    let numbers = [1, 2, 3];

    println!("{:?}", &numbers[..2]);
}

```
will show [2,3] and 

```rs
fn main() {
    // array
    let numbers = [1, 2, 3];

    println!("{:?}", &numbers[..2]);
}

```
will show [1,2]