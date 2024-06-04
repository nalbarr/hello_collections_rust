fn iterate_shape_strings_vector(shape_names: Vec<&str>) {
    println!("shapes_names: {:?}", shape_names);

    for name in &shape_names {
        match *name {
            "square1" => {
                println!("shape name is square");
            }
            "triangle1" => {
                println!("shape name is triangle");
            }
            "rectangle1" => {
                println!("shape name is rectangle");
            }
            _ => {
                panic!("unknown shape");
            }
        }
    }
}
#[derive(Debug)]
enum ShapeEnum {
    Square,
    Triangle,
    Rectangle,
}

fn iterate_shape_enums_vector(shape_enums: Vec<ShapeEnum>) {
    // NA
    // NOTE: ShapeEnum need Debug[] attribute for below
    println!("shapes_enums: {:?}", shape_enums);

    for shape_enum in &shape_enums {
        match shape_enum {
            ShapeEnum::Square => {
                println!("shape name is: {:?}", "square");
            }
            ShapeEnum::Triangle => {
                println!("shape name is: {:?}", "triangle");
            }
            ShapeEnum::Rectangle => {
                println!("shape name is: {:?}", "rectangle");
            }
        }
    }
}

fn main() {
    let shape_names: Vec<&str> = vec!["square1", "triangle1", "rectangle1"];
    iterate_shape_strings_vector(shape_names);

    let square: ShapeEnum = ShapeEnum::Square;
    let triangle: ShapeEnum = ShapeEnum::Triangle;
    let rectangle: ShapeEnum = ShapeEnum::Rectangle;
    let shape_enums: Vec<ShapeEnum> = vec![square, triangle, rectangle];
    iterate_shape_enums_vector(shape_enums);
}
