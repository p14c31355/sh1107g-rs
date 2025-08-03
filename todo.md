5. クレートの公開
cd sh1107g-driver
cargo publish 
--allow-dirty # もしgitの変更がコミットされていない場合
数分から数十分で crates.io と docs.rs にあなたのクレートが表示

oled_core として抽象化してみては？