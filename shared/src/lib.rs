#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod entities {
    use uuid::Uuid;

    #[derive(Clone, Debug)]
    pub struct Todo {
        pub id: Option<Uuid>,
        pub name: String
    }
}
