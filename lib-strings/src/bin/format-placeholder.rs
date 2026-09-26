fn main() {
    println!("{}富士 {}鷹 {}なすび", 1, 2, 3);
    //println!("{}富士 {}鷹 {}なすび", 1, 2, 3, 4);   // エラー
    //println!("{}富士 {}鷹 {}なすび", 1, 2);   // エラー

    println!("{2}富士 {0}鷹 {1}なすび", 2, 3, 1);
    //println!("{2}富士 {0}鷹 {1}なすび", 2, 3, 1, 4);    // エラー
    //println!("{2}富士 {0}鷹 {1}なすび", 2, 3);    // エラー

    println!(
        "1{fuji} 2{taka} 3{nasu}",
        fuji = "富士",
        taka = "鷹",
        nasu = "なすび"
    );
    //println!("1{fuji} 2{taka} 3{nasu}", fuji="富士", taka="鷹");    // エラー
    //println!("1{fuji} 2{taka} 3{nasu}", fuji="富士", taka="鷹", nasu="なすび", ebisu="えびす"); // エラー

    println!("1{0} 2{taka} 3{nasu}", "富士", taka = "鷹", nasu = "なすび");

    println!("{{〜}} はブロックです。");
}
