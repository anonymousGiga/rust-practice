mod person;
use person::Person;

fn main() {
    let alice = Person::new("alice", 20);
    alice.print();

    // let bob = Person{name: "bob".to_string(), age: 21};
    // bob.print();

    let default = Person::default();
    default.print();
}
