# Rust 言語基礎 document
1. [文字列 slice](#anchor1)
2. [String型](#anchor2)
3. [Vector型](#anchor3)
4. [Box pinter](#anchor4)
5. [ownership & borrowing](#anchor5)
    - 所有権　→　[所有と借用](#anchor5-1)
6. [Life time + Dangling pointer](#anchor6)
7. [Generics life time annotation](#anchor7)
## Rust App memory
<center>
<br>
<img src="source/images/app_memory.png"><br>
<br>
</center>


|                                                Memoryの中                                                |
| :------------------------------------------------------------------------------------------------------: |
| **Heap** (ヒープ)<br>容量が大きく可変長データを扱える(String, Vector)<br>memory access が stack より遅い |
|   **Stack** (スタック)<br>容量は限られているが高速な　Access ができる<br>sizeが決まった変数や配列など    |
|         **Static**　(静的領域)<br>const (global な定数を定義)<br>文字列リテラル(hard code)の実態         |
|                        **Text** (コード)<br>compile後のbinary code を保存する場所                        |


<center>
<br>
<img src="source/images/stack.png">
</center>

<center>
<br>
<h3>Last In Fast Out (Stack) 高速</h3>
<br>
push (入力)<br>
let で変数を定義（値の代入をバインドという）<br>
<br>
        ↓<br>
<br>
Memory<br>
<br>
        ↓<br>
<br>
pop (取り出す)<br>
スコープ ( { } : カーリーブラケット)を抜けると<br>
Memory の自動的に解放 (drop) される<br>
<br>
</center>

### 64bit os (横幅 8bytes)
- **pointer は memory内の番地(address)を表している**

  - *1byte ごとにaddressが割り振られている*

- 32bit os
  - 横幅 4bytes :

<center>
<br>
<img src="source/images/pointer.png"><br>
<br>
</center>

<a id="anchor1"></a>

## 文字列 slice
- Rust は utf-8 を採用している
  -  1 ~ 4 byte を自動的に割り当てていく

     - 英字 = **1byte**
     - 日本語 = **3byte**

- **ptr : 8bytes**　+　**len : 8bytes**　=　**s1(slice 変数) : 16bytes**
<center>
<br>
<img src="source/images/str_slice.png"><br>
<br>
</center>

### 参照
- **所有の権限が付与されない data型**

<center>
<br>
<img src="source/images/str_slice_reference1.png"><br>
<br>
</center>

<a id="anchor5-1"></a>

### 参照と借用

<center>
<br>
<img src="source/images/str_slice_reference2.png"><br>
<br>
</center>

<a id="anchor2"></a>

## String型

- ptr : 8bytes　+　len : 8bytes　+　**cap : 8bytes**　=　**s2(String 変数) : 24bytes**

    - *cap : capacity (実データが使用できる最大容量)*
      - rust が自動的に決めてくれる
      - lenth からある程度余裕を持った容量が設定されている
<center>
<br>
<img src="source/images/str_heap.png"><br>
<br>
</center>

### 所有権
- **二重開放 error 回避**

    - memory の解放ができる data型

<center>
<br>
<img src="source/images/str_heap_ownership.png"><br>
<br>
</center>

<a id="anchor3"></a>

## Vector型
 - String 型とほとんど同じ

  - ptr : 8bytes　+　**len : 要素数4(各4bytes)**　+　**cap :  要素数4(各4bytes)**　=　**s2(String 変数) : 24bytes**

    - ptr : heap 先頭の address と data型の情報を保持
    - len : bytes ではなく要素数 (各要素が4bytes)
    - cap : bytes ではなく要素数 (各要素が4bytes)

<center>
<br>
<img src="source/images/vector.png"><br>
<br>
</center>

<a id="anchor4"></a>

## Box pointer
- **Box pointer 自体は　= 8bytes** (※ 64bit os)

  - [List 列挙型の定義](https://doc.rust-lang.org/book/ch15-01-box.html#:~:text=which%20will%20compile%3A-,Filename%3A%20src/main.rs,-enum%20List%20%7B%0A%20%20%20%20Cons)

  - size が決まらないと compile できない(**Errorになる**)問題に対応できる
    - $\infty$ 無限 loop をさせない為に　**Box pointer で size を与える**　事によて解決できる

<center>
<br>
<img src="source/images/box_pointer.png"><br>
<br>
        ↓<br>
<br>
<img src="source/images/box_pointer2.png"><br>
<br>
</center>

<a id="anchor5"></a>

## 5. Ownership & Borrowing
[所有権について](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

### move

- **所有権を持つ型**
  - `String, Vector, Box pointer`　=　`所有権 move` が発生する
  - `所有権の移動`

  - **move**
    - **値を代入した時に, 代入した値だけではなく所有権も一緒に移譲される**

    - `move` *後は元の変数に access する事はできない*

  - **move が発生する処理**

    - `変数に copy した場合`
    - `関数の引数に与えた場合`
    - `関数の戻り値として与えた場合`

<center>
<br>
<img src="source/images/move.png"><br>
<br>
</center

### deep copy

<center>
<br>
<img src="source/images/move_d.png"><br>
<br>
</center>

### 5-1. 参照と借用
[参照と借用](#anchor5-1)

<center>
<br>
<img src="source/images/reference.png"><br>
<br>
</center

<a id="anchor6"></a>

## 6.Life time + Dangling pointer

[The book : 参照](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#:~:text=Listing%2010-19%20fixes%20the%20code%20so%20it%20doesn%E2%80%99t%20have%20a%20dangling%20reference%20and%20compiles%20without%20any%20errors)

### Life time

<center>
<br>
<img src="source/images/life_time.png"><br>
<br>
</center>

### Dangling pointer
- **dangling pointer = Error**
    - `compile` ができる　=　**dangling error はない**

<center>
<br>
<img src="source/images/dangling.png"><br>
<br>
</center>

<a id="anchor7"></a>

## 7.Generics life time annotation

<center>
<br>
<img src="source/images/refefence2.png"><br>
<br>
</center>

### Rust memory 安全性
- `RAII`　→　fileを開いた時も同じく, スコープを抜けると自動で close してくれる
  - **RAII は default 機能として Rust言語仕様となっている**

<center>
<br>
<img src="source/images/raii.png"><br>
<br>
</center>
