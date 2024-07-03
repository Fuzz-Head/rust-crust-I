//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[derive(Debug)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a>{ //just like genric impl<T> 
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self{ //not specifing lifetimes here causes complier to scream it means that the function variables could be dropped 
        //immediately after the function call and we are left with invalid memory locations to haystack 
        Self {
            remainder: Some(haystack),
            delimiter,   //field and value have same name so proper syntax
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;   //remainder if not mut then does not really point to the actual remainder from the struct 
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..]; // defrerencing and then assigning
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
            // _ = ""; has static lifetime and takes up memory when the program is launched  
    }
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