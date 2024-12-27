use std::any::Any;
use std::fmt::Debug;

pub trait Post: Any + Debug {
    fn post_construct(&self);
}