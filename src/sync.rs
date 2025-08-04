/// sync
#[cfg(feature = "sync")]
use embedded_hal::i2c::I2c;

#[cfg(feature = "sync")]
use crate::{cmds::*, BuilderError, Sh1107g, Sh1107gBuilder, DISPLAY_WIDTH};

#[cfg(feature = "sync")]
use core::result::Result;
#[cfg(feature = "sync")]
use core::result::Result::Ok;

// Sh1107g instance ( builded by builder ) call init and flush
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107gBuilder<I2C>
where
    I2C: embedded_hal::i2c::I2c<Error = E>,
{
    /// Build Sh1107g instance
    pub fn build(self) -> Result<Sh1107g<I2C>, BuilderError> {
        let i2c = self.i2c.ok_or(BuilderError::NoI2cConnected)?;
        // let size = self.size.ok_or(BuilderError::NoDisplaySizeDefined)?; // サイズが必須の場合

        // If you need, more add configure

        let oled = Sh1107g::new(i2c, self.address
            // size: size,
            // rotation: self.rotation,
            ); // Sh1107g::new init include buffer

        // display initialize やるかどうか
        Ok(oled)
    }
}

// Sh1107g impl block
#[cfg(feature = "sync")]
impl<I2C, E> Sh1107g<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Display initialize sequence
    // sync.rs または async_.rs

// cmds.rsからimportしている定数は、念のため以下でマジックナンバーに置き換えてください。
// 例: DISPLAY_OFF -> 0xAE
// ...

    pub fn init(&mut self) -> Result<(), E> {
        // 1. ディスプレイをオフにする
        self.send_cmd(0xAE)?;

        // 2. Display Clock Divide Ratio と Oscillator Frequency を設定 (Pythonで0x51)
        //    コマンドは0xD5, 引数は0x51
        self.send_cmdandarg(0xD5, 0x51)?;

        // 3. Multiplex Ratio を設定 (128x128ディスプレイでは 0x7F)
        //    コマンドは0xA8, 引数は0x7F
        self.send_cmdandarg(0xA8, 0x7F)?;

        // 4. Display Offset を設定 (Pythonで0x60)
        //    コマンドは0xD3, 引数は0x60
        self.send_cmdandarg(0xD3, 0x60)?;

        // 5. Display Start Line を設定 (リセット後のデフォルト値は0x00)
        //    このコマンドは0x40から0x7Fまでの範囲。
        //    0x40は「Set Display Start Line」コマンドそのものであり、引数を必要としません。
        //    そのため、0x40 + 0x00 = 0x40 を送ることで開始ライン0を設定します。
        self.send_cmd(0x40)?;

        // 6. Charge Pump を設定 (Pythonで0x8B)
        //    コマンドは0xAD, 引数は0x8B
        self.send_cmdandarg(0xAD, 0x8B)?;
        
        // 7. Segment Remap と COM Output Scan Direction を設定
        //    これはディスプレイの向きを制御する重要なコマンドです。
        //    0xA1: Segmentを反転 (Remapped)
        //    0xC8: COMスキャン方向を反転 (Remapped)
        self.send_cmd(0xA1)?;
        self.send_cmd(0xC8)?;
        
        // 8. COM Pins Hardware Configuration を設定
        //    コマンドは0xDA, 引数は0x12 (128x128では通常この値)
        self.send_cmdandarg(0xDA, 0x12)?;

        // 9. Contrast Control を設定 (0x2Fは一般的な値)
        //    コマンドは0x81, 引数は0x2F
        self.send_cmdandarg(0x81, 0x2F)?;

        // 10. VCOMH Deselect Level を設定
        //     コマンドは0xDB, 引数は0x35
        self.send_cmdandarg(0xDB, 0x35)?;

        // 11. Pre-charge Period を設定
        //     コマンドは0xD9, 引数は0x22
        self.send_cmdandarg(0xD9, 0x22)?;

        // 12. Entire Display On/Off を設定 (A4h: RAMの内容を通常表示)
        self.send_cmd(0xA4)?;

        // 13. Normal/Inverse Display を設定 (A6h: 通常表示)
        self.send_cmd(0xA6)?;

        // 14. メモリモードを設定 (Page Addressing Mode)
        //     コマンドは0x20, 引数は0x02
        self.send_cmdandarg(0x20, 0x02)?;

        // 15. ディスプレイをオンにする (初期化シーケンスの最後に配置)
        self.send_cmd(0xAF)?;

        Ok(())
    }

    /// IIC write command byte
    fn send_cmd(&mut self, cmd: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd])
    }

    /// IIC write command byte and data byte
    fn send_cmdandarg(&mut self, cmd: u8, arg: u8) -> Result<(), E> {
        self.i2c.write(self.address, &[0x00, cmd, arg])
    }

    /// Rendering
    // Send self internal buffer
    pub fn flush(&mut self) -> Result<(), E> {
        // SH1107G is page addressing mode and 128 byte/page
        // 128/8 = 16 page because 128x128 pixels
        for page in 0..16 { // 0 to 15
            self.send_cmd(0xB0 + page)?; // Set Page Address (B0h ~ BFh)
            self.send_cmd(0x00)?; // Set Lower Column Address (0x00)
            self.send_cmd(0x10)?; // Set Higher Column Address (0x10)

            // `buffer` は2048バイト全体で、各ページ128バイトなので
            // buffer[page * 128 .. (page + 1) * 128] で該当ページのスライスを取得

            // Cast to usize and perform calculation
            let page_usize = page as usize;
            let width_usize = DISPLAY_WIDTH as usize;

            let start_index = page_usize * width_usize;
            let end_index = (page_usize + 1) * width_usize;

            // スライスもusizeの範囲で指定
            // 内部バッファ保持
            let page_data = &self.buffer[start_index..end_index];

            // I2Cのwriteは1回の呼び出しで送信できるデータ量に制限がある場合があるため、
            // 16バイトずつ分割して送信するロジックは理にかなっている。
            // 各ページのデータを16バイトチャンクで送信
            for chunk in page_data.chunks(16) {
                let mut buf: heapless::Vec<u8, 17> = heapless::Vec::new(); // 制御バイト1 + データ最大16バイト
                buf.push(0x40).unwrap(); // control byte for data (0x40)
                buf.extend_from_slice(chunk).unwrap();
                self.i2c.write(self.address, &buf)?;
            }
        }
        Ok(())
    }
}
