fn main() {
    let a = divide_integer(9, 4);
    println!("{a}");
}

fn divide_integer(x: i32, y: i32) -> i32 {
    /* この記述だけだとコンパイルエラー
    if y != 0 {
        return x / y;
    }
    */
    if y != 0 {
        return x / y;
    } else {
        return 0;
    }
}

/*
fn divide_integer(x: i32, y: i32) -> i32 {
    if y != 0 {
        x / y
    } else {
        0
    }
}

fn divide_integer(x: i32, y: i32) -> i32 {
    if y != 0 {
        return x / y;
    }
    0
}

fn divide_integer(x: i32, y: i32) -> i32 {
    return if y != 0 {
        x / y
    } else {
        0
    };
}
*/
