/*
 * @@author        : xia_1  guo_114@outlook.com
 * @@date          : 2025-03-31 21:05
 * @@file          : \2025s-rustling-xianguo-1\exercises\options\options2.rs
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


// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
