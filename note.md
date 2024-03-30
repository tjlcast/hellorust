## cargo

cargo 与 rustc 的关系类似： maven 与 javac

## 变量绑定

``` rust
let (x, y) = (1i, 2i);
```

+ 模式匹配

``` rust
let x: int = 5;
```

+ 类型推演

``` rust
let y = 5i

let mut x = 5i;
x = 10i;
```

+ 默认，绑定是 不可变 的。For Safe
+ 使用mut

``` rust
fn foo(x: int) -> int {
    if x < 5 { return x; }

    x + 1
}
```
+ 分号将任意表达式都转换成语句，扔掉表达式原来的返回值，然后返回一个单元 ()。

## match

https://juejin.cn/post/7251895470547877943

``` rust
fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
}
```

+ 匹配枚举类型
+ 解构和匹配结构体
+ 使用 if let 简化匹配
+ 匹配多个模式: ｜
+ if let 和 while let


## 循环

+ loop

```
for (x = 0; x < 10; x++) {
    printf( "%d\n", x );
}
```

```
for x in range(0i, 10i) {
    println!("{}", x);
}
```

```
while !done {
    x += x - 3;
    println!("{}", x);
    if x % 5 == 0 { done = true; }
}
```

## 字符串

* Rust 有两种类型的字符串：&str 和 String

``` rust
let string = "Hello there.";
```
+ 第一种是 &str，念作 “字符串切片”。字面字符串都是 &str。这个字符串是静态分配的。静态分配的意思是，它存储在我们编译后的程序中，并且在程序运行的过程中一直存在着。string 绑定就是到这个静态字符串的引用。字符串切片有一个固定大小，不能被改变。

+ String 存在于内存中。这种字符串可以增长，但也被保证是 UTF-8 编码的。

+ 可以用 as_slice() 将 String 转化成 &str:


## 数组、向量和切片
+ 数组是一个固定大的列表，里面每个元素都有相同的类型。

+ [val; N] 这种语法创建一个指定数目的数组，每个元素都初始化成同样的值

``` rust
let a = [1i, 2, 3];     // Only the first item needs a type suffix

println!("a has {} elements", a.len());
for e in a.iter() {
    println!("{}", e);
}
```
+ 可以用 a.len() 得到数组的元素个数，用 a.iter() 来遍历一个数组

``` rust
let v = vec![1i, 2, 3];
```
+ 向量 是动态（或可变长度）的数组。由标准库 Vec<T> 实现


## Crates和模块

+ 一个 crate 是 Rust 的独立编译单元。Rust 总是一次编译一个 crate，产生一个库或可执行文件。然而，可执行文件依赖于库，而一些库有依赖于其它库。为了解决这个问题，crate 可以依赖于其它 crate。
+ 每个 crate 包含一组层级的模块。这个树以一个单一模块（叫做 crate 根）开始。在 crate 根上，我们可以声明其它模块，而那些模块又可以包含其它模块，随便多深都可以。


``` rust
fn main() {
    hello::print_hello();
}

mod hello {
    pub fn print_hello() {
        println!("Hello, world!")
    }
}
```

+ pub 关键字使用的时候，有时叫作 “导出”，因为我们使这个函数可以被其它模块使用

