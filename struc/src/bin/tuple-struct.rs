struct Ipv4Address(u8, u8, u8, u8);
struct ZipCode(u16, u16);

fn main() {
    let address = Ipv4Address(192, 168, 1, 1);
    println!(
        "IPv4 Address: {}.{}.{}.{}",
        address.0, address.1, address.2, address.3
    );
    let zipCode = ZipCode(225, 0002);
    println!("Zip Code: {:0>3}-{:0>4}", zipCode.0, zipCode.1);
}
