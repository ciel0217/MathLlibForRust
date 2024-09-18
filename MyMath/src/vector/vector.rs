use std::ops::{self, *};
use std::iter::{FromIterator};
pub trait VectorFunction
{
    fn length(&self) -> f64;
    fn normalize(&mut self) -> Self;
    fn dot(&self, rhs: &Self) -> f64;
}


pub struct Vector4
{
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64
}


impl FromIterator<f64> for Vector4
{
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
       
        let mut initer = iter.into_iter();
        Self{x : initer.next().unwrap(),y : initer.next().unwrap(),z : initer.next().unwrap(), w : initer.next().unwrap()}
        
    }
}

impl ops::Add<Vector4> for Vector4 {
    type Output = Self;
    fn add(self, rhs: Vector4) -> Self::Output {
        Self{x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z, w : self.w + rhs.w}
    }
}

impl ops::AddAssign<Vector4> for Vector4
{
    fn add_assign(&mut self, rhs: Vector4) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

impl ops::Mul<f64> for Vector4
{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self{x : self.x * rhs, y : self.y * rhs, z : self.z * rhs, w : self.w * rhs}
    }
}

impl ops::Mul<Vector4> for f64
{
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Self::Output {
        Vector4{x : rhs.x * self, y : rhs.y * self, z : rhs.z * self, w : self* rhs.w}
    }
}

impl ops::MulAssign<f64> for Vector4
{
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}



impl ops::Div<f64> for Vector4
{
    type Output = Vector4;
    
    fn div(self, rhs: f64) -> Self::Output {
        Self{x : self.x / rhs, y : self.y / rhs, z : self.z / rhs, w : self.w / rhs}  
    }
}

impl ops::DivAssign<f64> for Vector4
{
    fn div_assign(&mut self, rhs: f64)  {
    
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}

impl VectorFunction for Vector4
{
    fn length(&self) -> f64
    {
        (self.x*self.x + self.y * self.y + self.z * self.z + self.w*self.w).sqrt()
    }

    fn normalize(&mut self) -> Self {
        
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self.w /= length;
        
        Self{x : self.x, y : self.y, z : self.z, w : self.w}
    }

    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
}

