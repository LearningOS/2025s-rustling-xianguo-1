/*
 * @@author        : xia_1  guo_114@outlook.com
 * @@date          : 2025-03-31 21:05
 * @@file          : \learnd:\CODE\RUST\CAMP\2025s-rustling-xianguo-1\exercises\structs\structs2.rs
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


// structs2.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();
        // TODO: Create your own order using the update syntax and template above!
        // let your_order =
        let your_order = Order {
        name: String::from("Hacker in Rust"),
        count: 1, // 手动设置 count 为 1
        ..order_template // 其他字段从 order_template 继承
        };  
        
        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
