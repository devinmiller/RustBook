#[allow(dead_code)]
enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter(State)
}

#[derive(Debug)]
#[allow(dead_code)]
enum State {
    Alabama, 
    Alaska, 
    Arizona, 
    Arkansas, 
    California, 
    Colorado, 
    Connecticut, 
    Delaware, 
    DistrictOfColumbia, 
    Florida, 
    Georgia, 
    Hawaii, 
    Idaho, 
    Illinois, 
    Indiana, 
    Iowa, 
    Kansas, 
    Kentucky, 
    Louisiana, 
    Maine, 
    Maryland, 
    Massachusetts, 
    Michigan, 
    Minnesota, 
    Mississippi, 
    Missouri, 
    Montana, 
    Nebraska, 
    Nevada, 
    NewHampshire, 
    NewJersey, 
    NewMexico, 
    NewYork, 
    NorthCarolina, 
    NorthDakota, 
    Ohio, 
    Oklahoma, 
    Oregon, 
    Pennsylvania, 
    RhodeIsland, 
    SouthCarolina, 
    SouthDakota, 
    Tennessee, 
    Texas, 
    Utah, 
    Vermont, 
    Virginia, 
    Washington, 
    WestVirginia, 
    Wisconsin, 
    Wyoming
}

fn main() {
    let coin = Coin::Quarter(State::Arizona);

    println!("Value in cents: {}", value_in_cents(&coin));

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let quarter = Coin::Quarter(State::California);
    let dime = Coin::Dime;

    quarter_state(&quarter);
    quarter_state(&dime);
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn quarter_state(coin: &Coin) {
if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    }  else {
        println!("This is not a quarter!");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}