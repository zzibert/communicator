pub mod client {
    pub fn connect() {
    }
}

pub mod network {
    pub fn connect() {
    }
    pub mod server {
        pub fn connect() {
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
