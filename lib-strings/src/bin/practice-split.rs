fn main() {
    let str = "山田\t太郎\t男\t17歳\t新潟";
    let data = str.split("\t").collect::<Vec<_>>();
    for temp in data {
        println!("{temp}");
    }
}
