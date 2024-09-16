use std::ops::{*};

trait Length{ pub fn length(&self) -> f64;}
trait Normalize{ pub fn normalize(&mut self) -> Self;}
trait Cross{pub fn cross(&self, rhs : &Self) -> Self;}
trait Dot{pub fn dot(&self, rhs: &Self) -> f64;}

pub struct Vector4
{
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64
}

