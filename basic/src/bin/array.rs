use std::path::Path;

fn main() {
    // 配列の宣言(サイズ)
    let a: [i32; 5];

    // 配列の宣言(初期値)
    let a = [1, 2, 3, 4, 5];
    let s = ["弘法も筆の誤り", "猿も木から落ちる", "馬の耳に念仏"];
    let p = [
        Path::new(r#"c:¥users¥nao¥documents¥rust¥sample1.txt"#),
        Path::new(r#"c:¥users¥nao¥documents¥rust¥sample2.txt"#),
        Path::new(r#"c:¥users¥nao¥documents¥rust¥sample3.txt"#),
    ];
    let a: [i32; 5];
    a = [1, 2, 3, 4, 5];
    //let a: [i32; 5] = [1, 2, 3.0, 4, 5];	// 3.0はf64型なのでコンパイルエラー
    let a = [0; 10];

    // 配列へのアクセス
    println!("{}", a[0]);
    println!("{}", a.len());

    // 書き換え可能な配列の宣言
    let mut a = [1, 2, 3, 4, 5];
    a[0] = 100;
    println!("{}", a[0]);

    // 多次元配列
    let a: [[i32; 4]; 5];
    let a: [[[i32; 4]; 5]; 6];
    let a = [[1, 2, 3], [4, 5, 6]];
    let a: [[i32; 3]; 2];
    a = [[1, 2, 3], [4, 5, 6]];
    println!("{}", a[1][2]);
}
