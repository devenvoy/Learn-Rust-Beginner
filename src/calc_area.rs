// compiler does not know how to print custom struct type
#[derive(Debug)] // -> to print its bebug value to console using :? or :#? in {} 
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    let rect2 = Rectangle::square(50);

    // let area = calculate_area(&rect);
    let area = rect.calculate_area();
    println!("The area of {:?} is {}", rect, area);
    println!("The area of {:?} is {}", rect2, rect2.calculate_area());
}

/*
// this methos is not tied to Rectangle struct both are separate entity
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
*/

// # Implementation Block (impl)
// this way , lets use to define method for specific struct
// method define in this must have first param. &self ( which is reference to itself )
// &self -> self : &Self  [immutable]
// &mut self -> self : &mut Self  [mutable]
impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }

    // static function of stuct
    // this method do not take reference and can be used with ( :: ) operator and accessed without creating struct object
    // this are used directly from struc name with :: (operator) and method_name
    fn square(side: u32) -> Rectangle {
        Rectangle {
            width: side,
            height: side,
        }
    }
}
