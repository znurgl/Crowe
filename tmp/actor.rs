use rustc_serialize::json::Json;
use std::fmt::Debug;

pub trait Role {
    fn receive(message: Json) ;
}

