#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

impl UsState {
    fn exists_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1959,
            UsState::Alabama => year >= 1819,
        }
    }
}

enum Coin {
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}", state),
        _ => count += 1,
    };

    let coin2 = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin2 {
        println!("State quarter from {state:?} using if let");
    } else {
        count += 1;
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.exists_in(1900) {
    //         Some(format!("{state:?} is pretty old"))
    //     } else {
    //         Some(format!("{state:?} is pretty new"))
    //     }
    // } else {
    //     None
    // }
    //
    // let state = if let Coin::Quarter(state) = coin {
    // state
    // } else {
    //     return None;
    // }

    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.exists_in(1900) {
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is pretty new"))
    }
}
