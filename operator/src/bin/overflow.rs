fn main() {
    // コンパイル時にオーバフローが発生
    let a = 100_u8;
    let b = 200_u8;
    // let c = a + b; // コンパイルエラー
    // println!("{c}");

    // 実行時にオーバフローが発生
    let x = 100_u8;
    let y = 200_u8;
    #[allow(arithmetic_overflow)]
    let z = x + y; // コンパイルエラーにならないが実行時にpanic
    println!("{z}");
}
