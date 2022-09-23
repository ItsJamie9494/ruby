//! # Ruby
//! A crate for installing Linux distributions

#![deny(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

pub mod modules;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
