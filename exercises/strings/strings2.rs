/*
 * @@author        : xia_1  guo_114@outlook.com
 * @@date          : 2025-03-31 21:05
 * @@file          : \2025s-rustling-xianguo-1\exercises\strings\strings2.rs
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


// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}