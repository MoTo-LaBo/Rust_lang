# Rust 言語基礎
[Rust: The Book](https://doc.rust-lang.org/book/ch01-01-installation.html)

[Rust my document](https://github.com/MoTo-LaBo/rust_lang/blob/main/rust_lang.md#:~:text=Blame-,Rust%20%E8%A8%80%E8%AA%9E%E5%9F%BA%E7%A4%8E%20document,-Rust%20App%20memory)

- Rustの言語仕様
- 所有権・借用・Life time
- Generics・Traits・構造体・列挙型・ユーザー定義型
- Unit tet
- Rust beginner & basic
## Tree
```
    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── README.md
    ├── rust_lang.md
    ├── source
    │   └── images
    ├── src
    │   ├── basic
    │   │    ├──  main.rs
    │   │    ...
    │   └── bin
    │        ├── 01_func.rs
    │        ...
    └── target
        ├── CACHEDIR.TAG
        ├── debug
        └── rls
```
### update
    rustup update
### uninstall
    rustup self uninstall
### version
    rustc --version
### project 作成
    cargo new <project dir名>
- cargo package manager を使用して project dir を作成できる
### init command
    cargo init <option> <path>
- すでに存在している dir に対して Rust の構成 file を追加する事ができる
### build
    cargo build
### build & 実行
    cargo run
### 複数の実行 file
    cargo run --<dir> <name>
- **Cargo.toml file への記述を忘れずに　↓**

```
    [[bin]]
    name = "file名"
    path = "src/dir名/file名.rs
```
### 出力の非表示 option　**-q**
    cargo run -q --<dir> <name>
  - Compileing　→　ver, project path
  - Finished　→　実行時間
  - Runiing　→　実行 file path

    - `上記のを terminal から非表示にできる`　→　**実行結果だけを出力**
### code check
    cargo check
- code error check をしてくれる
  - cargo build でも error check はしてくれるが, バイナリーfile 生成の為時間がかかる
  - check はバイナリーfile は生成されない
### cargo fix
    cargo fix
- source code の中にある不備や改善点を発見し、修正を自動で適用してくれる
  - cargo check が裏で実行され, warning で修正可能なものは自動修正してくれる
### cargo edit install
    # install
    cargo install cargo-edit

    # Cargo.toml に追加
    cargo add

    # Cargo.toml から削除
    cargo rm

    # 最新バージョンにアップグレード
    cargo upgrade

    # Cargo.toml version 設定
    cargo set-version

[cargo edit](https://github.com/killercup/cargo-edit)

  - Cargo project 内で package の追加を簡単にしてくれる

    - **CLI で command 実行　→　自動で Cargo.toml file に追記してくれる**
