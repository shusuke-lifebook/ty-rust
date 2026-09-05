fn main() {
    // チェック付き除算
    let a: i32 = 100;
    let b: i32 = 0;
    if let Some(r) = a.checked_div(b) {
        println!("除算結果：{r}");
    } else {
        println!("0による除算");
    }

    // チェック付きオーバフロー
    let a = i32::MIN;
    let b = -1;
    if let Some(r) = a.checked_div(b) {
        println!("除算結果：{r}");
    } else {
        println!("除算オーバーフロー");
    }
}
