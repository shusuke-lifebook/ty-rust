fn main() {
    let mut i = 1;
    'vertical: while i < 10 {
        let mut j = 1;
        while j < 10 {
            let result = i * j;
            if result > 50 {
                break 'vertical;
            }
            print!("{result:2} ");
            j += 1;
        }
        println!();
        i += 1;
    }
    println!()
}
