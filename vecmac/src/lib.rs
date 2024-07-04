/* contains the entire code with counting macros 
    https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a
    
    link to youtube video 
    https://www.youtube.com/watch?v=q6paRBbLgNw&t=4088s
 */

#[macro_export]
macro_rules! avec {
    /*() => {
        Vec::new() /*again can be condensed */
    };
     ($element:expr) => {{
        let mut vs = Vec::new();
        vs.push($element);
        vs
    }};/*double brackets becasue the outer brackets define the rule of the macro */
    ($e1:expr, $e2:expr) => {{
        let mut vs = Vec::new();
        vs.push($e1);
        vs.push($e2);
        vs
    }};/* this is a poor mans function */
    */

    ($($element:expr),* /* $(,)? */) => {{
        #[allow(unused_mut)]
        let mut vs = Vec::new();
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::avec![$($element),*]
    }};

    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        /*let x = $element;
        for _ in 0..count {
            vs.push(x.clone());/*clone becasue it actually substitutes */
        }*/
        //vs.extend(std::iter::repeat($element).take(count));
        vs.resize($count, $element);
        vs
    }};
}

#[test]/*remove this line for Test and cargo expand to see derivatives */
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing_numbers() {
    let x: Vec<u32> = avec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        19, 20,
        ];
    assert!(!x.is_empty());
}

#[test]
fn trailing_characters() {
    let _ : Vec<&'static str> = avec![
        "adskjgfnsakldjgnskgnsklgnsgsdgs",
        "adskjgfnsakldjgnskgnsklgnsgsdgs",
        "adskjgfnsakldjgnskgnsklgnsgsdgs",
        "adskjgfnsakldjgnskgnsklgnsgsdgs",
        "adskjgfnsakldjgnskgnsklgnsgsdgs",
        ];
}

#[test]
fn clone_2 () {
    let x: Vec<u32> = avec![42; 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

#[test]
fn clone_nonliterals () {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}

/// ```compile_fail
///  let x: Vec<u32> = vecmac::avec![42, "foo"];
/// ```   
#[allow(dead_code)]
struct CompileFailTest;
/* try build is a good crate for CompileFailTest */

//cargo doc --open -> to show/create docs