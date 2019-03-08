use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec3<I = u32>(pub I, pub I, pub I);

impl<I> Vec3<I> {
    pub fn convert<J: From<I>>(self) -> Vec3<J> {
        Vec3(self.0.into(), self.1.into(), self.2.into())
    }

    pub fn map<J, F: Fn(I) -> J>(self, f: F) -> Vec3<J> {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    pub fn hadamard<J, K>(self, rhs: Vec3<J>) -> Vec3<K>
    where
        I: Mul<J, Output = K>,
    {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl<I: Default> Default for Vec3<I> {
    fn default() -> Self {
        Vec3(Default::default(), Default::default(), Default::default())
    }
}

impl<I: Clone> From<[I; 3]> for Vec3<I> {
    fn from(arr: [I; 3]) -> Vec3<I> {
        Vec3(arr[0].clone(), arr[1].clone(), arr[2].clone())
    }
}

impl<I> From<(I, I, I)> for Vec3<I> {
    fn from(t: (I, I, I)) -> Vec3<I> {
        Vec3(t.0, t.1, t.2)
    }
}

impl<I> Into<[I; 3]> for Vec3<I> {
    fn into(self) -> [I; 3] {
        [self.0, self.1, self.2]
    }
}

impl<I> Into<(I, I, I)> for Vec3<I> {
    fn into(self) -> (I, I, I) {
        (self.0, self.1, self.2)
    }
}

impl<I> From<Vec3<I>> for Vec2<I> {
    fn from(v: Vec3<I>) -> Vec2<I> {
        Vec2(v.0, v.1)
    }
}

impl<I, J, K> Add<Vec3<J>> for Vec3<I>
where
    I: Add<J, Output = K>,
{
    type Output = Vec3<K>;

    fn add(self, rhs: Vec3<J>) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<I, J, K> Sub<Vec3<J>> for Vec3<I>
where
    I: Sub<J, Output = K>,
{
    type Output = Vec3<K>;

    fn sub(self, rhs: Vec3<J>) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<I, J, K> Mul<J> for Vec3<I>
where
    I: Mul<J, Output = K>,
    J: Clone,
{
    type Output = Vec3<K>;

    fn mul(self, rhs: J) -> Self::Output {
        Vec3(self.0 * rhs.clone(), self.1 * rhs.clone(), self.2 * rhs)
    }
}

impl<I, J, K> Div<J> for Vec3<I>
where
    I: Div<J, Output = K>,
    J: Clone,
{
    type Output = Vec3<K>;

    fn div(self, rhs: J) -> Self::Output {
        Vec3(self.0 / rhs.clone(), self.1 / rhs.clone(), self.2 / rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2<I = u32>(pub I, pub I);

impl<I> Vec2<I> {
    pub fn convert<J: From<I>>(self) -> Vec2<J> {
        Vec2(self.0.into(), self.1.into())
    }

    pub fn map<J, F: Fn(I) -> J>(self, f: F) -> Vec2<J> {
        Vec2(f(self.0), f(self.1))
    }

    pub fn hadamard<J, K>(self, rhs: Vec2<J>) -> Vec2<K>
    where
        I: Mul<J, Output = K>,
    {
        Vec2(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<I: Default> Default for Vec2<I> {
    fn default() -> Self {
        Vec2(Default::default(), Default::default())
    }
}

impl<I: Clone> From<[I; 2]> for Vec2<I> {
    fn from(arr: [I; 2]) -> Vec2<I> {
        Vec2(arr[0].clone(), arr[1].clone())
    }
}

impl<I> From<(I, I)> for Vec2<I> {
    fn from(t: (I, I)) -> Vec2<I> {
        Vec2(t.0, t.1)
    }
}

impl<I> Into<[I; 2]> for Vec2<I> {
    fn into(self) -> [I; 2] {
        [self.0, self.1]
    }
}

impl<I> Into<(I, I)> for Vec2<I> {
    fn into(self) -> (I, I) {
        (self.0, self.1)
    }
}

impl<I, J, K> Add<Vec2<J>> for Vec2<I>
where
    I: Add<J, Output = K>,
{
    type Output = Vec2<K>;

    fn add(self, rhs: Vec2<J>) -> Self::Output {
        Vec2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<I, J, K> Sub<Vec2<J>> for Vec2<I>
where
    I: Sub<J, Output = K>,
{
    type Output = Vec2<K>;

    fn sub(self, rhs: Vec2<J>) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<I, J, K> Mul<J> for Vec2<I>
where
    I: Mul<J, Output = K>,
    J: Clone,
{
    type Output = Vec2<K>;

    fn mul(self, rhs: J) -> Self::Output {
        Vec2(self.0 * rhs.clone(), self.1 * rhs)
    }
}

impl<I, J, K> Div<J> for Vec2<I>
where
    I: Div<J, Output = K>,
    J: Clone,
{
    type Output = Vec2<K>;

    fn div(self, rhs: J) -> Self::Output {
        Vec2(self.0 / rhs.clone(), self.1 / rhs)
    }
}
