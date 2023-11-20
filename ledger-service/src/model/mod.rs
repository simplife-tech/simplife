use std::collections::HashMap;

lazy_static!{
    pub static ref LEDGER_CLAZZ_1: HashMap<i64, &'static str> = HashMap::from([
        (1, "Diet"),
        (2, "Live"),
        (3, "Goods"),
        (4, "Fixed"),
        (5, "Special")
    ]);

    pub static ref LEDGER_CLAZZ_2: HashMap<i64, &'static str> = HashMap::from([
        (1, "BaseFood"),
        (2, "BetterFood"),
        (3, "GreatFood"),
        (4, "Fruit"),
        (5, "Journey"),
        (6, "Daily"),
        (7, "BasicHealth"),
        (8, "BetterHealth"),
        (9, "Cloth"),
        (10, "WorkStudy"),
        (11, "Play"),
        (12, "Hobby"),
        (13, "Pet"),
        (14, "PetOthers"),
        (15, "MotherBaby"),
        (16, "Skill"),
        (17, "HomeBase"),
        (18, "Personal"),
        (19, "HouseRent"),
        (20, "HouseLoan"),
        (21, "CarLoan"),
        (22, "CarPark"),
        (23, "Water"),
        (24, "Electric"),
        (25, "Gas"),
        (26, "Heal"),
        (27, "TV"),
        (28, "Net"),
        (29, "Phone"),
        (30, "Study"),
        (31, "Insure"),
        (32, "OtherFix"),
        (33, "Favor"),
        (34, "Accident"),
        (35, "Charity"),
    ]);
}