fn main() {
    // 代入演算子
    let mut a = 100;
    println!("代入前：{a}");
    a = 200;
    println!("代入後：{a}");

    // 複合代入演算子
    let mut i = 100;
    let mut j = 200;
    println!("代入前：{i}, {j}");
    i += 10;
    j *= 10;
    println!("代入後：{i}, {j}");

    // 副作用のある複合代入演算子
    let i = 100;
    let mut j = 200;
    println!("代入前：{i}, {j}");
    j += {
        j = 0;
        i
    };
    println!("代入後：{i}, {j}");

    // 代入演算子の評価結果
    let mut a = 100;
    let b = (a = 200);
    println!("{:?}", b);

    // 配列型の代入
    let mut ary: [i32; 5] = [1, 2, 3, 4, 5];
    ary = [6, 7, 8, 9, 10];
    // ary = [6, 7, 8, 9, 10, 11]; // コンパイルエラー

    // タブルの代入
    let mut t = (1, 5.0, 'A');
    t = (2, 10.0, 'Z');
    // t = (3, 'X', 2.0, true); // エラー
}
