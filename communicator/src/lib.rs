// mod network {
//     fn connect() {
//     }

//     mod client {
//         fn connect() {
//         }
//     }
// }
// if we want to call this from outside network module we use network::connect()
// and network::client::connect()

// first structure
// communicator
// |-network
// |-client

// second structure
// communicator
// |- network
//     |- client

// mod client {
//     fn connect() {
//     }
// }
/////////////////
// communicator
// |- client
// |- network
//      |- server
pub mod client;

pub mod network;


#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // ::client::connect(); // starts from root
        // super::client::connect(); // moves up one module
        client::connect();
    }
}
