macro_rules! test_eq {
    ($name:ident, $left:expr, $midddle:expr, $right:expr) => {
        #[test]
        fn $name() {
            assert_eq!($left * $midddle, $right);
        }
    }
}

test_eq!(testcasenotequal, 10,  2, 30);
test_eq!(testcaseequal, 10,  2, 20);

fn main() {
}