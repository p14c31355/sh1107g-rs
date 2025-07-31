3. Crates.io アカウントの準備
Crates.io アカウントの作成: crates.io にアクセスし、GitHubアカウントでログインします。

APIトークンの取得: ログイン後、Account Settings ページから新しいAPIトークンを生成します。このトークンはクレートを公開するために必要です。

トークンの設定: ローカルのPCで以下のコマンドを実行し、取得したトークンを設定します。

Bash

cargo login <APIトークン>
（一度設定すれば、同じPCでは再度設定する必要はありません）

4. 公開前の最終確認
テスト:

アプリケーションクレート (avr-arduino-uno) からあなたの sh1107g-driver を実際に使ってみて、意図通りに動作することを確認します。

embedded-graphics を統合した場合は、それを使った描画もテストします。

ドキュメント生成:

cargo doc --open でローカルにドキュメントを生成し、表示が崩れていないか、説明が十分かなどを確認します。

README.md の充実:

クレートの目的、インストール方法、基本的な使用例、APIの概要などを分かりやすく記述します。

バッジ（crates.ioのバージョン、docs.rsのビルドステータスなど）を追加すると見栄えが良くなります。

ライセンスファイルの配置:

sh1107g-driver/LICENSE-MIT と sh1107g-driver/LICENSE-APACHE ファイルが正しく配置されていることを確認します。

cargo check / cargo clippy:

コードの文法エラーや推奨されない書き方がないか確認します。

cargo clippy --workspace --all-targets --all-features

5. クレートの公開
sh1107g-driver ディレクトリに移動し、以下のコマンドを実行します。

Bash

cd sh1107g-driver
cargo publish --allow-dirty # もしgitの変更がコミットされていない場合
cargo publish コマンドは、クレートをビルドし、必要なメタデータ（Cargo.toml の情報）とソースコードを crates.io にアップロードします。

--allow-dirty は、ローカルのGitリポジトリにコミットされていない変更がある場合でも公開を許可するオプションです。通常はクリーンな状態で公開することが推奨されます。

公開が成功すると、数分から数十分で crates.io と docs.rs にあなたのクレートが表示されるようになります。

ファイル分割とコードの移動:

src/common.rs を作成し、Sh1107g 構造体の定義、Sh1107gBuilder の定義と共通の実装、DrawTarget と Dimensions トレイトの実装を移動。

src/sync.rs を作成し、同期版の Sh1107g メソッド（init, flush, send_command_single, send_command_with_arg）の impl ブロックを移動。

src/async.rs を作成し、非同期版の Sh1107g メソッドの impl ブロックを移動 (async fn と await を追加)。

src/lib.rs でこれらのモジュールを #[cfg] を使って宣言し、公開するAPIを pub use する。

BuilderError の From 実装も #[cfg] で分岐させる。

必要であれば、ディスプレイコマンドの定数をまとめる src/commands.rs を作成し、各モジュールから利用する。

I²Cトレイトの境界とメソッドシグネチャの調整:

各 impl ブロックで、適切な embedded_hal::i2c::I2c または embedded_hal_async::i2c::I2c トレイトを境界として指定。

非同期メソッドには async キーワードと await を追加。