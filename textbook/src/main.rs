fn main() {
    trait Location {
        fn address(&self) -> &str;
    }

    trait Person {
        fn name(&self) -> &str;
    }

    trait House: Location + Person {}

    fn print_house_info(house: &dyn House) {
        println!("This house is located at {}", house.address());
        println!("The owner of this house is {}", house.name());
    }

    struct MyHouse {
        address: String,
        owner: String,
    }
    impl Location for MyHouse {
        fn address(&self) -> &str {
            &self.address
        }
    }
    impl Person for MyHouse {
        fn name(&self) -> &str {
            &self.owner
        }
    }
    impl House for MyHouse {}

    let my_house = MyHouse {
        address: "123 Main St".to_string(),
        owner: "John Doe".to_string(),
    };
    print_house_info(&my_house);
}
