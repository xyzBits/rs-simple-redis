use enum_dispatch::enum_dispatch;
use std::ops::Deref;

#[enum_dispatch(Hello)]
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Gender {
    Male(Male),
    Female(Female),
}

pub trait Hello {
    fn hello(&self);
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Male(String);

impl Male {
    pub fn new(name: &str) -> Male {
        Male(name.into())
    }
}

impl Deref for Male {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hello for Male {
    fn hello(&self) {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Female {}

impl Hello for Female {
    fn hello(&self) {
        todo!()
    }
}

impl From<Female> for Gender {
    fn from(value: Female) -> Self {
        Gender::Female(value)
    }
}

#[test]
fn test_enum_into() {
    // 实现了还是不行
    // let gender: Gender = Male::new("tom").into();
}
