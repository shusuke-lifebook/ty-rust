fn main() {
    let mut i = 1;
    'vertical: loop {
        if i >= 10 {
            break;
        }
        let mut j = 1;
        loop {
            if j >= 10 {
                break;
            }
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
    println!();
}
