strings

concatenation string

```rs
fn main() {
    let l0 = 'b';
    let l1 = 'r';
    let l2 = 'u';
    let l3 = 'n';
    let l4 = 'o';

    println!("{l0}{l1}{l2}{l3}{l4}")
}

```

will show bruno


heap string or dynamic string or String 

```rs
fn main() {
    let mut s = String::new();
    s.push('b');
    s.push('r');
    s.push('u');
    s.push('n');
    s.push('o');
    println!("{s}");
}

```

will show bruno


you can push a full string like

```rs

fn main() {
    let mut s = String::new();
    s.push_str("Bruno");
    s.push_str(" ");
    s.push_str("Rocha");
    println!("{s}");
}

```

will show Bruno Rocha


```rs
fn main() {
    let s = "Bruno".to_string();

    println!("{s}");
}

```

will transform to dynamic string and print Bruno

```rs
fn main() {
    let s = String::from("Bruno");

    println!("{s}");
}

```
will transform to dynamic string and print Bruno



from a iterable array to string need use String::from_iter();


```rs

fn main() {
    let nome = ['b', 'r', 'u', 'n', 'o'];
    let s = String::from_iter(nome);
    println!("{s}");
}
```

will print bruno


to transform to type predefined need use into() like 


```rs
fn main() {
    let s: String = "Bruno".into();
    println!("{s}");
}

```

will print bruno and need type or trow error


and owned 

```rs
fn main() {
    let s: String = "Bruno".into();
    println!("{s}");
}

```
will print Bruno with a onwnership




libraries 


```rs

use std::io;

fn main() {
    let mut s = String::new();
    println!("Digite um texto");

    io::stdin()
        .read_line(&mut s) // Result with onwnership
        .expect("Error reading console");

    println!("Vocề digitou {s}")
}
```

will print on termial Digite um texto and you need type something and press enter after that will show "Vocề digitou something"