fn main() {
    // 短絡評価
    let a = 0;
    let b = 3;
    //if (a != 0) & (b % a == 0) {  // &では実行時エラー
    if a != 0 && b % a == 0 {
        println!("{a} は　{b} で割り切れます。");
    }

    let a = false;
    let b = 3;
    if !a || b > 0 {
        println!("{} は {} で割り切れます。", b, a);
    }

    // 副作用のある条件式
    let mut a = 0;
    let b = 3;
    if a == 0
        || b % {
            a += 2;
            a
        } == 0
    {
        println!("{} は {} で割り切れます。", b, a);
    }
}
