// structs1.rs
// Address all the TODOs to make the tests pass!

/**
 * Structs are similar to tuples, which were discussed in “The Tuple Type” section.
 * Like tuples, the pieces of a struct can be different types.
 * Unlike with tuples, you’ll name each piece of data so it’s clear what the values mean.
 * As a result of these names, structs are more flexible than tuples:
 * you don’t have to rely on the order of the data to specify or access the values of an instance.
 */

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        let green = ColorClassicStruct {
            name: String::from("green"),
            hex: String::from("#00FF00"),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}
