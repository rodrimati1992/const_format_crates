use cfmt::{assertc, assertc_eq, assertc_ne};

// uninferred argument
assertc!(false, "{}", 0);

assertc!(true, "{}");

assertc!(true, "{}", foo = "", 100u8);

assertc_eq!(0u8, [0u8][10]);

assertc_eq!(0, 0, "{}", 0u8);

assertc_eq!((), (), "{}");

assertc_eq!((), (), "{}", foo = "", 100u8);

assertc_eq!(0u8, 1u8, "{}", 0);
