use std::boxed::Box;
use std::option::Option;
use std::result::Result;
use Maybe::*;

#[derive(Debug, PartialEq, Eq)]
enum Maybe<T> {
    Nothing,
    Just(T),
}

// Let's see at Haskell definition about Functor
// class Functor F where
//     fmap :: (a -> b) -> F a -> F b
// Using Trait generic to copycat this definition
// A == a
// B == b
// Self == F a
// Output == F b
// F == (a -> b)
pub trait Functor<'a, A, B, F>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output;
    fn fmap(&'a self, f: F) -> Self::Output;
}

pub fn fmap<'a, A, B, X, F>(x: &'a X, f: F) -> X::Output
    where
        F: Fn(&'a A) -> B,
        X: Functor<'a, A, B, F> {
    x.fmap(f)
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Maybe<A>
    where
        A: 'a,
        F: Fn(&'a A) -> B {

    type Output = Maybe<B>;
    fn fmap(&'a self, f: F) -> Maybe<B> {
        match *self {
            Just(ref x) => Just(f(x)),
            Nothing => Nothing,
        }
    }
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Box<A>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output = Box<B>;
    fn fmap(&'a self, f: F) -> Box<B> {
        Box::new(f(&**self))
    }
}

impl<'a, A, B, F> Functor<'a, A, B, F> for Option<A>
    where
        A: 'a,
        F: Fn(&'a A) -> B {
    type Output = Option<B>;
    fn fmap(&'a self, f: F) -> Option<B> {
        self.as_ref().map(|x| f(&x))
    }
}

impl<'a, A, B, E, F> Functor<'a, A, B, F> for Result<A, E>
    where
        A: 'a,
        E: Copy,
        F: Fn(&'a A) -> B {
    type Output = Result<B, E>;
    fn fmap(&'a self, f: F) -> Result<B, E> {
        match *self {
            Ok(ref x) => Ok(f(x)),
            Err(e) => Err(e)
        }
    }
}

#[test]
fn it_works_with_maybe() {
    let just = Just(7);
    let nothing = fmap(&Nothing, |x| x + 1);
    let other = fmap(&just, |x| x + 1);
    assert_eq!(nothing, Nothing);
    assert_eq!(other, Just(8));
}

#[test]
fn it_works_with_box() {
  let ax = Box::new(0);
  let bx = fmap(&ax, |a| a + 1);
  let cx = fmap(&ax, |a| a + 2);
  assert_eq!(bx, Box::new(1));
  assert_eq!(cx, Box::new(2));
}

#[test]
fn it_works_with_option() {
  let ax = Option::Some(0);
  let bx = fmap(&ax, |a| a + 1);
  let cx = fmap(&ax, |a| a + 2);
  assert_eq!(bx, Option::Some(1));
  assert_eq!(cx, Option::Some(2));
}

#[test]
fn it_works_with_result() {
  let ax: Result<_, ()> = Result::Ok(0);
  let bx = fmap(&ax, |a| a + 1);
  let cx = fmap(&ax, |a| a + 2);
  assert_eq!(bx, Result::Ok(1));
  assert_eq!(cx, Result::Ok(2));
}
