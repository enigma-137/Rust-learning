enum IpAddressKind {
v4,
v6,
}

struct IpAddress {
    kind: IpAddressKind,
    address: String
} 
fn main() {
    // specify variances
    let four = IpAddressKind::v4;
    let five = IpAddressKind::v6;
// create the variable for the local host ip address
    let localhost = IpAddress {
        kind: IpAddressKind::v4,
        address: String::from("127.0.0.1"),
    };
}

fn route(ip_kind: IpAddressKind){

}
