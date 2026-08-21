fn main() {
    // i32型の最小値を設定
    let i = i32::MIN;
    println!("{:032b}", i);
    println!("{:032b}", i >> 5);
    // println!("{:032b}", i as u32 >> 5);

    // i32型の型をu32型にキャスト
    let u = i as u32;
    println!("{:032b}", u);
    println!("{:032b}", u >> 5);

    // 以降、未使用

    // ビットシフト
    let a = 0b00010101;
    let s = 3;
    println!("{0} ➡ {0:08b} << {1} ➡ {2:08b} ➡ {2}", a, s, a << s);
    println!("{0} ➡ {0:08b} >> {1} ➡ {2:08b} ➡ {2}", a, s, a >> s);

    //println!("{0} ➡ {0:08b} << {1} ➡ {2:08b} ➡ {2}", a, s, a << -1);
    //println!("{0} ➡ {0:08b} >> {1} ➡ {2:08b} ➡ {2}", a, s, a >> -1);
}
