fn main() {
    let mut i = 1;
    while i < 10 {
        let mut j = 1;
        while j < 10 {
            let result = i * j;
            if result > 40 {
                break;
            }
            print!("{result:2} ");
            j += 1;
        }
        println!();
        i += 1;
    }
}
