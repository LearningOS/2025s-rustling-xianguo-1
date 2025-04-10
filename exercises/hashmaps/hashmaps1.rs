/*
 * @@author        : xia_1  guo_114@outlook.com
 * @@date          : 2025-03-31 21:05
 * @@file          : \2025s-rustling-xianguo-1\exercises\hashmaps\hashmaps1.rs
 * @@name          : 
 * @@details       : 
 * @@version       : 1.0
 * @@par           : null
 * @@warning       : FBI Warning!
 * @copyright    : Copyright(c) 2025 by xia_1, All Rights Reserved. 
 * 
 * *******************coding=utf-8*******************
 * *******************鲲鲲保佑无bug*******************
 * @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%*++#%@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@#=-::::-+%@@@@@@@@@
 * @@@@@@@@@@@@@@@@@@@@@#*+=-#%+##:...:-::::*@@@@@@@@
 * @@@@@@@@@@@@@@@@@@@#-.....=%...:::.=*=-:.=@@@@@@@@
 * @@@@@@@@@@@@@@@@@@+....::..%=.:....:+*+++*@@@@@@@@
 * @@@@@@@@@@@@@@@@#: ........#* ......:#@@@@@@@@@@@@
 * @@@@@@@@@@@@@@@=  . .......=#-:......=@@@@@@@@@@@@
 * @@@@@@@@@@@@@@+ ...+:......-*-.--. .. +@@@@@@@@@@@
 * @@@@@@@@@@@@@@@- ..:.......#= ..-++-. :@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@= . ..... -#.... .-**--@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@@= ...... #+ ..... .-+#@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@@@*:.... .*. ......  .+@@@@@@@@@@@
 * @@@@@@@@@@@@@@@@*+*++=--:-=........  =%@@@@@@@@@@@
 * @@@@@@@@@@@@@@@%=++++========--::.  -@@@@@@@@@@@@@
 * @@@@@@@@@@@@@%#+++++=============-+%@@@@@@@@@@@@@@
 * @@@@@@@@@@@%*+++++================+*@@@@@@@@@@@@@@
 * @@@@@@@@@%*+++++===-------------====@@@@@@@@@@@@@@
 * @@@@@@@@#+======--=+*#%%%%#+=---====*@@@@@@@@@@@@@
 * @@@@@@@*+===--=+#%@@@@@@@@@@@*--=====*@@@@@@@@@@@@
 * @@@@@@#++=--*#@@@@@@@@@@@@@@@@%+-----=#@@@@@@@@@@@
 * @@@@@@*+++=#@@@@@@@@@@@@@@@@@@@@#+--=++%@@@@@@@@@@
 * @@@@@%+++==@@@@@@@@@@@@@@@@@@@@@@@%++++*@@@@@@@@@@
 * @@@@@%+++=*@@@@@@@@@@@@@@@@@@@@@@@@*=+++%@@@@@@@@@
 * @@@@@#++=+@@@@@@@@@@@@@@@@@@@@@@@@@%=+++*@@@@@@@@@
 * @@@@@#++=%@@@@@@@@@@@@@@@@@@@@@@@@@@#=+++%@@@@@@@@
 * @@@@@+=-*@@@@@@@@@@@@@@@@@@@@@@@@@@@@#=++%@@@@@@@@
 * @@@@%:::@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@+::+@@@@@@@@
 * @@@@=.-:*@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%:-:*@@@@@@@
 * @@%+::::-@@@@@@@@@@@@@@@@@@@@@@@@@@@@@%--::*@@@@@@
 */


// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

// hashmaps1.rs
//
// A basket of fruits in the form of a hash map needs to be defined. The key
// represents the name of the fruit and the value represents how many of that
// particular fruit is in the basket. You have to put at least three different
// types of fruits (e.g apple, banana, mango) in the basket and the total count
// of all the fruits should be at least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a
// hint.


fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();// TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.
    basket.insert(String::from("apple"), 1);
    basket.insert(String::from("orange"), 3);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
