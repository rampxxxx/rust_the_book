/* Not work for structs but for enums
struct Message{
quit:(),
moves : { x:i32, y:i32},
write: String,
change_color: (i32,i32,i32),
*/
enum Message {
    Quit,
    Moves { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKindWithValue {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    /* Also valid.
    enum IpAddrKind {
        V4,
        V6,
    }
    */
    //
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(IpAddrKind::V4);
    //
    let four_valued = IpAddrKindWithValue::V4(127, 0, 0, 1);
    let six_valued = IpAddrKindWithValue::V6(String::from("::1"));
    route_valued(four_valued);
    route_valued(IpAddrKindWithValue::V4(0, 0, 0, 0));
    // Standard library ip's already defined
    // not bringed those in scope yet.

    //
    calc_cents();
}

fn route(ip: IpAddrKind) -> bool {
    true
}

fn route_valued(ip: IpAddrKindWithValue) -> bool {
    true
}

//
#[derive(Debug)]
enum EuState {
    Spain,
    Germany,
}

#[derive(Debug)]
enum Coin {
    OneCent,
    TwoCent,
    FiveCent,
    TenCent(EuState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::OneCent => 1,
        Coin::TwoCent => 2,
        Coin::FiveCent => 5,
        Coin::TenCent(state) => {
            // Cannot embedded match into match ?
            /*
            match state {
                EuState::Germany => {
                    println!("State germany\n");
                    1
                }
                EuState::Spain => {
                    println!("State spain\n");
                    1
                }
            };
            */
            println!("State {:?}!\n", state);
            10
        }
    }
}

fn calc_cents() {
    value_in_cents(Coin::OneCent);
    value_in_cents(Coin::TenCent(EuState::Germany));
}
