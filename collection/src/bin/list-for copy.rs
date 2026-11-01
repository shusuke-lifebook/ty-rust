fn main() {
    let vec = Vec::from([
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ]);
    for month in &vec {
        println!("{month}");
    }
    let mut itr = vec.iter();
    while let Some(month) = itr.next() {
        println!("{month}");
    }

    let mut vec = Vec::from([
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ]);
    for month in &mut vec {
        *month = month.to_uppercase();
        println!("{month}");
    }
    let mut itr = vec.iter_mut();
    while let Some(month) = itr.next() {
        *month = month.to_uppercase();
        println!("{month}");
    }
    println!("{:?}", vec);

    let vec = Vec::from([
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ]);
    for month in vec {
        println!("{month}");
    }
    //println!("{:?}", vec);    // 所有権が移動するので使えない
    let vec = Vec::from([
        "January".to_string(),
        "February".to_string(),
        "March".to_string(),
    ]);
    let mut itr = vec.into_iter();
    while let Some(mut month) = itr.next() {
        month = month.to_uppercase();
        println!("{month}");
    }
}
