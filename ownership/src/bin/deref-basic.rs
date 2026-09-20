fn main() {
    let a = 123;
    let r1 = &a;
    let r2 = r1;
    let v = *r1;
    println!("aは {a}、r1は {r1}、r2は {r2}、vは {v} です。");
    println!("aは {a}、r1は {}、r2は {}、vは {v} です。", *r1, *r2);

    let mut b = 456;
    let r3 = &mut b;
    *r3 += 1;
    println!("r3は {r3} です。");
}
