# Rust 言語基礎 document

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
let で変数を定義（値の代入をバインドという）
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

## 文字列の slice と String型
### 文字列 slice
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

#### 参照
- **所有の権限が付与されない data型**

<center>
<br>
<img src="source/images/str_slice_reference1.png"><br>
<br>
</center>

#### 参照と借用

<center>
<br>
<img src="source/images/str_slice_reference2.png"><br>
<br>
</center>


### String型

- ptr : 8bytes　+　len : 8bytes　+　**cap : 8bytes**　=　**s2(String 変数) : 24bytes**

    - *cap : capacity (実データが使用できる最大容量)*
      - rust が自動的に決めてくれる
      - lenth からある程度余裕を持った容量が設定されている
<center>
<br>
<img src="source/images/str_heap.png"><br>
<br>
</center>

#### 所有権
- **二重開放 error 回避**

    - memory の解放ができる data型

<center>
<br>
<img src="source/images/str_heap_ownership.png"><br>
<br>
</center>
