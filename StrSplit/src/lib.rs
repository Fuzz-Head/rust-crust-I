//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs, non_snake_case)]
/* coding tutorials where the code keeps changing on explanation or get refactored and improvised is hard to capture 
  this does include the important aspects, some things may not be covered properly and can be referenced at 
  video link https://www.youtube.com/watch?v=rAl-9HwD858&t=3491s 
  and source at https://gist.github.com/jonhoo/2a7fdcf79be03e51a5f95cd326f2a1e8
  this file however contains the last changed and refactored code */

#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D>{ //just like genric impl<T> 
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self{ //not specifing lifetimes here causes complier to scream it means that the function variables could be dropped 
        //immediately after the function call and we are left with invalid memory locations to haystack 
        Self {
            remainder: Some(haystack),
            delimiter,   //field and value have same name so proper syntax
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where 
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;   //remainder if not mut then does not really point to the actual remainder from the struct 
        if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
            let until_delimiter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..]; // defrerencing and then assigning
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
            // _ = ""; has static lifetime and takes up memory when the program is launched  
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}


fn until_char(s: &str, c: char) -> &'_ str {
    StrSplit::new(s, c)/*the complier always considers the shorter lifetime */
        .next()
        .expect("StrSplit always gives at least one result")
}/* this function would fail with a single lifetime, since we use two different lifetimes the function and the rust complier can infer it correctly */

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello World", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
    //or 
    let chars: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(chars, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn tails() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
    //or 
    let chars: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(chars, vec!["a", "b", "c", "d", ""]);
}