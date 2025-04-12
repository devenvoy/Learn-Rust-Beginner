pub fn struct_demo() {
    let user1 = User {
        active: true,
        user_name: String::from("Devansh"),
        email: String::from("devansh.argon@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        user_name: String::from("ArcEnvoy"),
        // ..user1
        ..user1.clone()
    };

    user1.print_user();
    println!();
    user2.print_user();
}

// owned typed Struct
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u8,
}
impl User {
    fn clone(&self) -> User {
        User {
            user_name: self.user_name.clone(),
            email: self.email.clone(),
            sign_in_count: self.sign_in_count,
            active: self.active,
        }
    }

    fn print_user(&self) {
        println!("{}", self.active);
        println!("{}", self.user_name);
        println!("{}", self.email);
        println!("{}", self.sign_in_count);
    }
}

// tuple struct 
struct Color(u8,u8,u8);

struct Point(u8,u8,u8);

// even if both are same we cannot use both each alternative 
/*
tuples are fixed-length arrays where each element has a known type. Sometimes, we want to restrict a tuple to only specific types and lengths, and prevent similar-looking tuples from being used unintentionally. This improves type safety.
 */

 struct Unit ;
 // this is simialr to () [Empty tuple]