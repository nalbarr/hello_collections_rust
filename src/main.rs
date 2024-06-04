/*
enum Shape {
    Square { side: f64 },
    Triangle { base: f64, height: f64 },
}
*/

/*
fn iterate_vector(shapes: &Vec<&str>) {}
*/

/*
fn iterate_vector_deq(shapes: @Vec<&str>) {
    for (i: usize, item: &&str) in shapes.iter().enumerate() {
        if i != shapes.len() - 1 {
            println!("{}", item);
        } else {
            println!("{}", item);
        }
    }
}
*/
fn main() {
    let shape_names: Vec<&str> = vec!["square1", "triangle1", "rectangle1"];
    println!("shapes_names: {:?}", shape_names);

    for name in &shape_names {
        println!("shape_name: {:?}", name);
        match *name {
            "square1" => {
                println!("shape name is: {}", "square");
            }
            "triangle1" => {
                println!("shape name is: {}", "triangle");
            }
            "rectangle1" => {
                println!("shape name is: {}", "");
            }
            _ => {
                panic!("unknown shape name");
            }
        }
    }
}
