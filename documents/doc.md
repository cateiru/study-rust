# Rust Documents

## 変数と定数

Rustでは変数は基本的に不変である。

```rust
fn main() {
  let x = 5;
  println!("The value of x is: {}", x);
  x = 6; // Error: cannot assign twice to immutable variable `x`
  println!("The value of x is: {}", x);
}
```

mutを使用することで可変になる。

```rust
fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);
}
```

### 変数と定数の違い

|                             定数                             |    変数     |
| :----------------------------------------------------------: | :---------: |
|                     `mut`は使用できない                      | `mut`使用可 |
|                            const                             |     let     |
| 定数式のみに設定でき、関数呼び出しの結果などは設定できない。 |             |

const 例:

```rust
const MAX_PORNTS: u32 = 100_000;
```

### シャドーイング

```rust
fn main() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("The value of x is: {}", x); // The value of x is: 12
}
```

注意: ただし型を変更することは許可されていない。

## データ型

- 整数型

  |  長さ   | 符号付き | 符号なし |
  | :-----: | :------: | :------: |
  |  8 bit  |   `i8`   |   `u8`   |
  | 16 ibt  |  `i16`   |  `u16`   |
  | 32 bit  |  `i32`   |  `u32`   |
  | 64 bit  |  `i64`   |  `u64`   |
  | 128 bit |  `i128`  |  `u128`  |
  |  arch   | `isize`  | `usize`  |

  |  数値リテラル  |      例       |
  | :------------: | :-----------: |
  |     10進数     |   `98_222`    |
  |     16進数     |    `0xfff`    |
  |     8進数      |    `0o77`     |
  |     2進数      | `0b1111_0000` |
  | bite(`u8`のみ) |    `b'A'`     |

- 浮動小数点型

  `f32`, `f64`。デフォルトは`f64`。

  ```rust
  fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
  }
  ```

- 数値演算

  ```rust
  fn main() {
    // +
    let sum = 5 + 10;
    // -
    let diff = 95.5 - 4.3;
    // mul
    let product = 4 * 30;
    // div
    let quptient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
  }
  ```

- ブール型

  ```rust
  fn main() {
    let t = true;
    let f: bool = false;
  }
  ```

- char型

  4バイトであり、Unicodeで表す。

  ```rust
  fn main() {
    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';
  }
  ```

- タプル

  一回宣言すると長さを変えることはできない。

  ```rust
  fn main() {
      let tup: (i32, f64, u8) = (500, 6.4, 1);
  }
  ```

  ```rust
  fn main() {
      let tup = (500, 6.4, 1);
  
      let (x, y, z) = tup;
  
      println!("The value of y is: {}", y); // The value of y is: 6.4
  }
  ```

  ```rust
  fn main() {
      let x: (i32, f64, u8) = (500, 6.4, 1);
  
      let five_hundred = x.0; // 500
  
      let six_point_four = x.1; // 6.4
  
      let one = x.2;  // 1
  }
  ```

- Array

  配列のすべての要素は同じ型である必要がある。

  ```rust
  fn main() {
      let a = [1, 2, 3, 4, 5];
    	let b: [i32; 5] = [1, 2, 3, 4, 5];
    	let c = [3; 5]; // c = [3, 3, 3, 3, 3]
    
    	let first = a[0]; // 1
      let second = a[1]; // 2
  }
  ```

## 関数

命名規則: スネークケース

