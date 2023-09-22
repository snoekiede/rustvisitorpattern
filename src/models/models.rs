pub trait Visitor {
    fn visit_person(&self,person:&Person);
    fn visit_organization(&self,o:&Organization);
}

pub trait Element {
    fn accept(&self,visitor:&mut dyn Visitor);
}

pub struct Person {
    name: String,
    email: String,
}

impl Person {
    pub(crate) fn new(name: &str, email: &str) ->Self {
        Self {
            name: name.to_string(),
            email: email.to_string()
        }
    }
}

impl Element for Person {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_person(self);
    }
}



pub struct Organization {
    name: String,
    address: String,
}

impl Element for Organization {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_organization(self);
    }
}

impl Organization {
    pub fn new(name: &str,address:&str)->Self {
        Self {
            name: name.to_string(),
            address: address.to_string()
        }
    }
}

pub struct EmailVisitor;

impl Visitor for EmailVisitor {
    fn visit_person(&self, person: &Person) {
        println!("Sending email to {} at {}",person.name,person.email);
    }

    fn visit_organization(&self, o: &Organization) {
        println!("Send mail to {} at {}",o.name,o.address);
    }
}


