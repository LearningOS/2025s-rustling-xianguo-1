/*
 * @@author        : xia_1  guo_114@outlook.com
 * @@date          : 2025-03-31 21:05
 * @@file          : \RUST\CAMP\2025s-rustling-xianguo-1\exercises\structs\structs1.rs
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


// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.



struct ColorClassicStruct {
    // TODO: Something goes here
    red: i32,
    green: i32,
    blue: i32,
}   

struct ColorTupleStruct( i32,i32,i32/* TODO: Something goes here */);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // let green =
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        // let green =
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit-like struct!
        // let unit_like_struct =
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
