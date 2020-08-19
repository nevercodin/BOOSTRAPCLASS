# Musical Sound Generator Framework (MSGF) in Rust

This software is released under the MIT License, see LICENSE.txt.

## 外部環境との接続

- "cargo build" で静的ライブラリが作成されます
- ライブラリをC言語でコールするための msgf.h が用意されています
- 私自身は Xcode で、Swift+ObjectiveC によるMacのコンソールアプリを作成し、そこからこのRu