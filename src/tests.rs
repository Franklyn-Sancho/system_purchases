/* #[cfg(test)]
mod tests {
    use crate::Saldo;

    #[test]
    fn test_add_coins() {
        let mut saldo = Saldo::new();
        saldo.add_coins(0.25);
        assert_eq!(saldo.saldo, 0.25);
        saldo.add_coins(1.00);
        assert_eq!(saldo.saldo, 1.25);
    }

    #[test]
    fn test_buy_product() {
        let mut item = Saldo::new();
        item.add_coins(1.00);
        item.buy_product("biscoito");
        assert_eq!(item.saldo, 0.0);
        item.add_coins(1.00);
        item.buy_product("soda");
        assert_eq!(item.saldo, 0.0);
    }
} */