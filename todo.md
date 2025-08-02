Account Settings ページから新しいAPIトークンを生成します。
cargo login <APIトークン>
（一度設定すれば、同じPCでは再度設定する必要はありません）

4. 公開前の最終確認
テスト:

ドキュメント生成:

cargo doc --open

README.md の充実:

クレートの目的、インストール方法、基本的な使用例、APIの概要などを分かりやすく記述します。

バッジ（crates.ioのバージョン、docs.rsのビルドステータスなど）を追加すると見栄えが良くなります。


cargo clippy --workspace --all-targets --all-features

5. クレートの公開
sh1107g-driver ディレクトリに移動し、以下のコマンドを実行します。

cd sh1107g-driver
cargo publish --allow-dirty # もしgitの変更がコミットされていない場合
cargo publish コマンドは、クレートをビルドし、必要なメタデータ（Cargo.toml の情報）とソースコードを crates.io にアップロードします。

--allow-dirty は、ローカルのGitリポジトリにコミットされていない変更がある場合でも公開を許可するオプションです。通常はクリーンな状態で公開することが推奨されます。

数分から数十分で crates.io と docs.rs にあなたのクレートが表示

refactoring

src/async.rs を作成し、非同期版の Sh1107g メソッドの impl ブロックを移動 (async fn と await を追加)。

src/lib.rs でこれらのモジュールを #[cfg] を使って宣言し、公開するAPIを pub use する。

BuilderError の From 実装も #[cfg] で分岐させる。


I²Cトレイトの境界とメソッドシグネチャの調整:

各 impl ブロックで、適切な embedded_hal::i2c::I2c または embedded_hal_async::i2c::I2c トレイトを境界として指定。

非同期メソッドには async キーワードと await を追加。

oled_core として抽象化してみては？