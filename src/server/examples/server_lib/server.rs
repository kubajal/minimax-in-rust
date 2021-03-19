//! Server implementation of swagger_client.

#![allow(unused_imports)]

use futures::{self, Future};
use chrono;

use std::collections::HashMap;

use std::marker::PhantomData;

use swagger;
use swagger::{Has, XSpanIdString};

use swagger_client::{Api, ApiError,
                      ComputerMoveResponse,
                      GetCurrentBoardStateResponse
};
use swagger_client::models;

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}

impl<C> Api<C> for Server<C> where C: Has<XSpanIdString>{

    /// Computer move
    fn computer_move(&self, board: models::Board, context: &C) -> Box<Future<Item=ComputerMoveResponse, Error=ApiError>> {
        let context = context.clone();
        println!("computer_move({:?}) - X-Span-ID: {:?}", board, context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

    /// Get board state
    fn get_current_board_state(&self, context: &C) -> Box<Future<Item=GetCurrentBoardStateResponse, Error=ApiError>> {
        let context = context.clone();
        println!("get_current_board_state() - X-Span-ID: {:?}", context.get().0.clone());
        Box::new(futures::failed("Generic failure".into()))
    }

}
