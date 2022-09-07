pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    //use crate::client;

    #[test]
    fn it_works() {
        crate::client::connect();
    }
}
