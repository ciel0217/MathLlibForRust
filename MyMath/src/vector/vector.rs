
use std::ops::{self, *};
use std::iter::{FromIterator};
pub trait VectorFunction
{
    fn length(&self) -> f64;
    fn normalize(&mut self) -> Self;
    fn dot(&self, rhs: &Self) -> f64;
}

//------------------------------------------------------Vector4------------------------//
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



///------------------------------------Vector3------------------------------------------//


pub struct Vector3
{
    x : f64,
    y : f64,
    z : f64
}

impl ops::Add<Vector3> for Vector3
{
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        Self{x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z}
    }
}

impl ops::AddAssign<Vector3> for Vector3
{
    fn add_assign(&mut self, rhs: Vector3) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl ops::Sub<Vector3> for Vector3
{
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Self{x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z}
    }
}

impl ops::SubAssign<Vector3> for Vector3
{
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

impl ops::Mul<Vector3> for Vector3
{
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Self{x : self.x * rhs.x, y : self.y * rhs.y, z : self.z * rhs.z}
    }
}

impl ops::Mul<f64> for Vector3
{
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self{x : self.x * rhs, y : self.y * rhs, z : self.z * rhs}
    }
}

impl  ops::MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl ops::MulAssign<f64> for Vector3
{
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3{x : self * rhs.x, y : self * rhs.y, z : self * rhs.z}
    }
}

impl VectorFunction for Vector3
{
    fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn length(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    fn normalize(&mut self) -> Self {
        let length = self.length();
        Self{x : self.x / length, y : self.y / length, z : self.z / length}
    }
}

impl Vector3
{
    fn cross(self, rhs : Vector3) -> Vector3
    {
        Vector3{
            x : self.y * rhs.z - self.z * rhs.y,
            y : -(self.x * rhs.z - self.z * rhs.x),
            z : self.x * rhs.y - self.y * rhs.x
        }
    }
}

//----------------------------Vector2-------------------------

pub struct Vector2
{
    x : f64,
    y : f64
}

impl ops::Add<Vector2> for Vector2
{
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2{x : self.x + rhs.x, y : self.y + rhs.y}
    }
}

impl ops::AddAssign<Vector2> for Vector2
{
    fn add_assign(&mut self, rhs: Vector2) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl ops::Sub<Vector2> for Vector2
{
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2{
            x : self.x - rhs.x,
            y : self.y - rhs.y
        }
    }
}

impl ops::SubAssign<Vector2> for Vector2
{
    fn sub_assign(&mut self, rhs: Vector2) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl ops::Mul<Vector2> for Vector2
{
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2
        {
            x : self.x * rhs.x,
            y : self.y * rhs.y
        }
    }
}


