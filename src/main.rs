#[derive(Debug)]
struct Admin {
    username: &'static str
}

impl User for Admin {
    fn new(username: &'static str) -> Admin {
        Admin {username}
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Ususário do tipo ADMIN entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Ususário do tipo ADMIN saiu do sistema"
    }
}

#[derive(Debug)]
struct Operador {
    username: &'static str
}

impl User for Operador {
    fn new(username: &'static str) -> Operador {
        Operador {username}
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Ususário do tipo OPERADOR entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Ususário do tipo OPERADOR saiu do sistema"
    }
}

struct MyStruct {
    x: i32
}

impl PartialEq for MyStruct {
    fn eq(&self, other: &MyStruct) -> bool {
        self.x == other.x
    }
}

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let admin: Admin = User::new("Clayton");

    println!("Bem vindo usuário {}", admin.username);
    println!("{}", admin.login());
    println!("{}", admin.logout());

    let operador = Operador::new("Marcelo");

    println!("Bem vindo usuário {}", operador.username);
    println!("{}", operador.login());
    println!("{}", operador.logout());

    println!();
    println!("{:#?}", admin);
    println!("{:#?}\n", operador);

    let n1 = 1;
    let n2 = 2;

    println!("n1 == n2   -> {}", n1 == n2);
    println!("n1.eq(&n2) -> {}", n1.eq(&n2));
    println!("n1 != n2   -> {}", n1 != n2);
    println!("n1.ne(&n2) -> {}\n", n1.ne(&n2));

    println!("n1 == n2   -> {}", 1 == 2);
    println!("n1.eq(&n2) -> {}", 1.eq(&2));
    println!("n1 != n2   -> {}", 1 != 2);
    println!("n1.ne(&n2) -> {}\n", 1.ne(&2));

    let a = MyStruct {x: 10};
    let b = MyStruct {x: 20};

    println!("{}", a == b);

    let p1 = Point {x: 1, y: 2};
    let p2 = Point {x: 3, y: 4};

    println!("{}", p1 != p2);
}

trait User {
    fn new(username: &'static str) -> Self;

    fn username(&self) -> &'static str;

    fn login(&self) -> &'static str;

    fn logout(&self) -> &'static str;

    fn is_logged_in(&self) -> bool {
        false
    }
}
