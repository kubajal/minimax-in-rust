#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


extern crate futures;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// Logically this should be in the client and server modules, but rust doesn't allow `macro_use` from a module.
#[cfg(any(feature = "client", feature = "server"))]
#[macro_use]
extern crate hyper;

extern crate swagger;

use futures::Stream;
use std::io::Error;

#[allow(unused_imports)]
use std::collections::HashMap;

pub use futures::Future;

#[cfg(any(feature = "client", feature = "server"))]
mod mimetypes;

pub use swagger::{ApiError, ContextWrapper};

pub const BASE_PATH: &'static str = "/v0";
pub const API_VERSION: &'static str = "0.0.1";


#[derive(Debug, PartialEq)]
pub enum ComputerMoveResponse {
    /// successful operation
    SuccessfulOperation ( object ) ,
}

#[derive(Debug, PartialEq)]
pub enum GetCurrentBoardStateResponse {
    /// successful operation
    SuccessfulOperation ( object ) ,
}


/// API
pub trait Api<C> {

    /// Computer move
    fn computer_move(&self, board: models::Board, context: &C) -> Box<Future<Item=ComputerMoveResponse, Error=ApiError>>;

    /// Get board state
    fn get_current_board_state(&self, context: &C) -> Box<Future<Item=GetCurrentBoardStateResponse, Error=ApiError>>;

}

/// API without a `Context`
pub trait ApiNoContext {

    /// Computer move
    fn computer_move(&self, board: models::Board) -> Box<Future<Item=ComputerMoveResponse, Error=ApiError>>;

    /// Get board state
    fn get_current_board_state(&self) -> Box<Future<Item=GetCurrentBoardStateResponse, Error=ApiError>>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<'a, C> where Self: Sized {
    /// Binds this API to a context.
    fn with_context(self: &'a Self, context: C) -> ContextWrapper<'a, Self, C>;
}

impl<'a, T: Api<C> + Sized, C> ContextWrapperExt<'a, C> for T {
    fn with_context(self: &'a T, context: C) -> ContextWrapper<'a, T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

impl<'a, T: Api<C>, C> ApiNoContext for ContextWrapper<'a, T, C> {

    /// Computer move
    fn computer_move(&self, board: models::Board) -> Box<Future<Item=ComputerMoveResponse, Error=ApiError>> {
        self.api().computer_move(board, &self.context())
    }

    /// Get board state
    fn get_current_board_state(&self) -> Box<Future<Item=GetCurrentBoardStateResponse, Error=ApiError>> {
        self.api().get_current_board_state(&self.context())
    }

}

#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use self::client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

pub mod models;
