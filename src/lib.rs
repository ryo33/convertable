#![no_std]

use core::convert::Infallible;

pub trait ConvertFrom<T>: Sized {
    #[must_use]
    fn convert_from(value: T) -> Self;
}

pub trait ConvertInto<T>: Sized {
    #[must_use]
    fn convert_into(self) -> T;
}

pub trait TryConvertFrom<T>: Sized {
    type Error;

    fn try_convert_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryConvertInto<T>: Sized {
    type Error;

    fn try_convert_into(self) -> Result<T, Self::Error>;
}

// ConvertFrom implies ConvertInto
impl<T, U> ConvertInto<U> for T
where
    U: ConvertFrom<T>,
{
    fn convert_into(self) -> U {
        U::convert_from(self)
    }
}

// TryConvertFrom implies TryConvertInto
impl<T, U> TryConvertInto<U> for T
where
    U: TryConvertFrom<T>,
{
    type Error = U::Error;

    fn try_convert_into(self) -> Result<U, Self::Error> {
        U::try_convert_from(self)
    }
}

// ConvertInto implies TryConvertFrom
impl<T, U> TryConvertFrom<U> for T
where
    U: ConvertInto<T>,
{
    type Error = Infallible;

    fn try_convert_from(value: U) -> Result<Self, Self::Error> {
        Ok(U::convert_into(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! structs {
        () => {
            #[derive(Debug, PartialEq)]
            pub struct A(u8);
            #[derive(Debug, PartialEq)]
            pub struct B(u8);
            #[derive(Debug, PartialEq)]
            pub struct C<T>(T);
        };
    }

    #[test]
    fn convert_from_implies_convert_into() {
        structs!();
        impl ConvertFrom<A> for B {
            fn convert_from(value: A) -> Self {
                Self(value.0)
            }
        }
        let actual: B = A(1).convert_into();
        assert_eq!(actual, B(1));
    }

    #[test]
    fn try_convert_from_implies_try_convert_into() {
        structs!();
        impl TryConvertFrom<A> for B {
            type Error = ();

            fn try_convert_from(value: A) -> Result<Self, Self::Error> {
                Ok(Self(value.0))
            }
        }
        let actual: Result<B, ()> = A(1).try_convert_into();
        assert_eq!(actual, Ok(B(1)));
    }

    // #[test]
    // fn into_implies_convert_from() {
    //     structs!();
    //     impl Into<B> for A {
    //         fn into(self) -> B {
    //             B(self.0)
    //         }
    //     }
    //     assert_eq!(B::convert_from(A(1)), B(1));
    // }

    // #[test]
    // fn try_into_implies_try_convert_from() {
    //     structs!();
    //     impl TryInto<B> for A {
    //         type Error = ();
    //         fn try_into(self) -> Result<B, Self::Error> {
    //             Ok(B(self.0))
    //         }
    //     }
    //     assert_eq!(B::try_convert_from(A(1)), Ok(B(1)));
    // }

    #[test]
    fn convert_into_implies_try_convert_from() {
        structs!();
        impl ConvertInto<B> for A {
            fn convert_into(self) -> B {
                B(self.0)
            }
        }
        let actual: Result<B, Infallible> = B::try_convert_from(A(1));
        assert_eq!(actual, Ok(B(1)));
    }

    #[test]
    fn convert_container() {
        structs!();
        impl Into<B> for A {
            fn into(self) -> B {
                B(self.0)
            }
        }
        impl<T: Into<U>, U> ConvertFrom<C<T>> for C<U> {
            fn convert_from(value: C<T>) -> Self {
                Self(value.0.into())
            }
        }
        let actual: C<B> = C::convert_from(C(A(1)));
        assert_eq!(actual, C(B(1)));
    }

    #[test]
    fn try_convert_container() {
        structs!();
        impl TryInto<B> for A {
            type Error = ();
            fn try_into(self) -> Result<B, Self::Error> {
                Ok(B(self.0))
            }
        }
        impl<T: TryInto<U, Error = E>, U, E> TryConvertFrom<C<T>> for C<U> {
            type Error = E;
            fn try_convert_from(value: C<T>) -> Result<Self, Self::Error> {
                Ok(Self(value.0.try_into()?))
            }
        }
        let actual: Result<C<B>, ()> = C::try_convert_from(C(A(1)));
        assert_eq!(actual, Ok(C(B(1))));
    }
}
