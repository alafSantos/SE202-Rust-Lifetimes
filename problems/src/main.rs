use std::ops::{Deref, DerefMut};

fn ret_string() -> String {
    String::from("  A String object  ")
}

// fn choose_str(s1: &str, s2: &str, select_s1: bool) -> &str {
// introducing lifetime parameters
/**
 * When I'm introducing a lifetime parameter, should I
 * have two parameters (e.g. 'a and 'b) ? if so, how should I deal with the return ?
 */
fn choose_str<'a>(s1: &'a str, s2: &'a str, select_s1: bool) -> &'a str {
    let result;
    if select_s1 {
        result = s1;
    } else {
        result = s2;
    }
    return result as &'a str;
}

// Write a OOR enum with two alternatives: Owned which stored a String and Borrowed which stores a &str.
// It will require using a generic parameter. What does it represent?
/**
 * 'a is a generic parameter used to represent a lifetime parameter
 * which specifies that the Borrowed is borrowing a string for the lifetime of the OOR.
 * */
enum OOR<'a> {
    Owned(String),
    Borrowed(&'a str),
}

// Implement the Deref trait for the OOR structure so that it dereferences into an a &str.
// What is the lifetime of the resulting &str (note that you have no choice there)?
// Why is that always appropriate?
/*
   The impl keyword is used to associate a type with a trait and to provide
   the implementation of the trait's methods for that type.
*/
impl Deref for OOR<'_> {
    type Target = str;

    /*
    Self is a special keyword that refers to the type of the current implementation or the current trait.
    It is equivalent to writing the type name explicitly.

    On the other hand, self is a normal identifier that refers to the current instance of a type.
    It is used to access fields or methods on the instance, or to pass the instance as a parameter to a method.
     */
    fn deref(&self) -> &Self::Target {
        match self {
            OOR::Owned(s) => (*s).as_str(),
            OOR::Borrowed(s) => *s,
        }
    }
}

// Write a DerefMut trait for the OOR structure.
// If you have not stored a String, you will have to mutate and store a String
// before you can hand out a &mut str because you can't transform your inner &str into &mut str.
impl DerefMut for OOR<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            OOR::Owned(s) => s.as_mut_str(),
            OOR::Borrowed(s) => {
                *self = OOR::Owned(s.to_string());
                match self {
                    OOR::Owned(s) => s,
                    OOR::Borrowed(_) => unreachable!(), // unreachable! is just a shorthand for panic! with a fixed, specific message
                }
            }
        }
    }
}

fn main() {
    /*
     * The trim method on str (and thus on String thanks to Deref)
     * removes the blanks at the beginning and at the end of a string.
     */
    // temporary value dropped while borrowed
    // let s = ret_string().trim(); // creates a temporary value which is freed while still in use
    let s = ret_string().trim().to_string(); // &str to String
    assert_eq!(s, "A String object");

    // -------------------------------------------
    // use case of OOR type
    let s1 = String::from("str1");
    let s2 = "str2";
    let oor1 = OOR::Owned(s1);
    let oor2 = OOR::Borrowed(s2);

    assert_eq!(oor1.len(), 4);
    assert_eq!(oor1.to_uppercase(), "STR1");

    assert_eq!(oor2.len(), 4);
    assert_eq!(oor2.to_uppercase(), "STR2");

    // -------------------------------------------
    // Check Deref for both variants of OOR
    let s1 = OOR::Owned(String::from("  Hello, world.  "));
    assert_eq!(s1.trim(), "Hello, world.");
    let mut s2 = OOR::Borrowed("  Hello, world!  ");
    assert_eq!(s2.trim(), "Hello, world!");

    // Check choose
    let s = choose_str(&s1, &s2, true);
    assert_eq!(s.trim(), "Hello, world.");
    let s = choose_str(&s1, &s2, false);
    assert_eq!(s.trim(), "Hello, world!");

    // Check DerefMut, a borrowed string should become owned
    assert!(matches!(s1, OOR::Owned(_)));
    assert!(matches!(s2, OOR::Borrowed(_)));
    unsafe {
        for c in s2.as_bytes_mut() {
            if *c == b'!' {
                *c = b'?';
            }
        }
    }
    assert!(matches!(s2, OOR::Owned(_)));
    assert_eq!(s2.trim(), "Hello, world?");
}
