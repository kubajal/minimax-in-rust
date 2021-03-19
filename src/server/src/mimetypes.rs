/// mime types for requests and responses

pub mod responses {
    use hyper::mime::*;

    // The macro is called per-operation to beat the recursion limit
    /// Create Mime objects for the response content types for ComputerMove
    lazy_static! {
        pub static ref COMPUTER_MOVE_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }
    /// Create Mime objects for the response content types for GetCurrentBoardState
    lazy_static! {
        pub static ref GET_CURRENT_BOARD_STATE_SUCCESSFUL_OPERATION: Mime = "application/json".parse().unwrap();
    }

}

pub mod requests {
    use hyper::mime::*;
   /// Create Mime objects for the request content types for ComputerMove
    lazy_static! {
        pub static ref COMPUTER_MOVE: Mime = "application/json".parse().unwrap();
    }

}
