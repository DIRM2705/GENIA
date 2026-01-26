use std::ops;
use std::hash::Hash;

#[derive(Clone, Copy)]
pub struct OrderedF64(f64);

impl Hash for OrderedF64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        if self.0.is_nan() {
            // Hash NaN as a fixed value
            0u64.hash(state);
        } else {
            // Use the bit representation of the f64 for hashing
            self.0.to_bits().hash(state);
        }
    }
}

impl PartialEq for OrderedF64 {
    fn eq(&self, other: &Self) -> bool
    {
        if self.0.is_nan() || other.0.is_nan() {
            return false;
        }
        return self.0 == other.0;
    }

    fn ne(&self, other: &Self) -> bool
    {
        !self.eq(other)
    }
}

impl Eq for OrderedF64 {}

impl ops::Add<OrderedF64> for OrderedF64 
{
    type Output = OrderedF64;

    fn add(self, rhs: OrderedF64) -> Self::Output 
    {
        OrderedF64(self.0 + rhs.0)
    }
}

impl ops::Div<OrderedF64> for OrderedF64 
{
    type Output = OrderedF64;

    fn div(self, rhs: OrderedF64) -> Self::Output 
    {
        OrderedF64(self.0 / rhs.0)
    }
}

impl ops::Sub<OrderedF64> for OrderedF64 
{
    type Output = OrderedF64;

    fn sub(self, rhs: OrderedF64) -> Self::Output 
    {
        OrderedF64(self.0 - rhs.0)
    }
}

impl ops::Mul<OrderedF64> for OrderedF64 
{
    type Output = OrderedF64;

    fn mul(self, rhs: OrderedF64) -> Self::Output 
    {
        OrderedF64(self.0 * rhs.0)
    }
}

impl Into<f64> for OrderedF64 
{
    fn into(self) -> f64 
    {
        self.0
    }
}

impl OrderedF64 
{
    pub const fn abs(self) -> OrderedF64
    {
        return OrderedF64(self.0.abs());
    }
}
