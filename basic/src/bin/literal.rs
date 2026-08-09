fn main() {
    // 整数リテラル
    let i = 2147483647;
    // let i = 2147483648; // コンパイルエラー
    let u = 2147483648u32;
    let l = 2147483648i64;
    // 浮動小数点リテラル
    let f = 3.4028235e38f32;

    // 文字リテラル
    let c = 'あ';
    let uc = '\u{32ff}';
    let cc = '\t';
    let qc = '\'';

    // 文字列リテラル
    let str = "Hello, world!";

    // バイト文字列リテラル
    let b = b"Hello, world!";

    // C言語文字列リテラル（ヌル終端文字列リテラル）
    let c = c"Hello, world!";

    // 文字エスケープ
    let str = "Hello, \"Yamada-san\"!";

    // 生文字列リテラル
    let str1 = r#"Hello, "Yamada-san"!"#;
    let str2 = r##"Number is #045-123-4567"##;
}
