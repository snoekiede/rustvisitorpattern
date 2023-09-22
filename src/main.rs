use crate::models::models::{Element, EmailVisitor, Organization, Person};

mod models;

fn main() {
    let mut elements:Vec<Box<dyn Element>>=Vec::new();

    let alice=Box::new(Person::new("Alice","alice@example.com"));
    let bob=Box::new(Person::new("Bob", "bob@example.com"));
    let acme=Box::new(Organization::new("Acme Inc.","123 Main Str."));

    elements.push(alice);
    elements.push(acme);
    elements.push(bob);

    let mut email_visitor=EmailVisitor;

    for element in elements {
        element.accept(&mut email_visitor);
    }


}
