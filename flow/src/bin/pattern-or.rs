fn main() {
    let month = "Jun";
    let days = match month {
        "Jan" | "Mar" | "May" | "Jul" | "Aut" | "Oct" | "Dec" => 31,
        "Apr" | "Jun" | "Sep" | "Nov" => 30,
        "Feb" => 28,
        _ => 0,
    };
    println!("{month} has {days} days.");
}
