//! builds the tree dictionary to be used as the spell checker 

use std::io::{Read, BufReader, BufRead, stdin, Write, stdout};
use std::collections::HashMap;

type Trie = Vec<Node>;

pub struct Node {
	letter: char,
	children: Trie,
}



impl<'a> Node {
	pub fn new(letter: char) -> Self {
	 	Node {
			letter: letter,
			children: vec![],
		}
	}
}


pub struct SpellChecker {
    counts: HashMap<String, isize>,
    cache: Trie,
    sum: f64,
}

fn printer(cache: &Trie, level: usize) {
	for node in cache {
		println!("Node's letter is {} and level of Trie is {}", node.letter, level);
		printer(&node.children, level+1);
	}
}

impl SpellChecker {
	 pub fn new<R: Read>(stream: R) -> Self {
        let input = BufReader::new(stream);
        let mut sum = 0.;
        let mut counts = HashMap::new();
        let mut cache: Trie = vec![];

        for line in input.lines() {
            for token in line.unwrap().split(" ") {
                if token == "" { continue; }
                add_to_trie(&mut cache, token);
                // build_cache_node(&mut counts, &mut cache, token);
                *counts.entry(String::from(token)).or_insert(0) += 1;
                sum += 1.;
            }
        }

        // println!("Building Trie . . .\n");
        // printer(&cache, 0);
        // println!("\n");
       
        SpellChecker { counts: counts, cache: cache, sum: sum }
    }

    

   fn probability(&self, word: &String) -> f64 {
        match self.counts.get(word) {
            Some(occurances) => *occurances as f64 / self.sum,
            None => 0.
        }
    }

    /// Returns a correction for an incorrect word
    pub fn correct(&mut self, word: String) -> String {
        let mut max = 0.;
        let mut correction = String::from("-");

        // Get all potential edits
        let edit1c = self.candidates(word);


        let mut edit2c = Vec::new();
        for c in edit1c {
            if self.wordp(&c)  { edit2c.push(c.clone());  }// add edit 1 c? !!! 
            edit2c.extend(self.candidates(c));
        } 


        // Find best edit
        for c in edit2c.iter().filter(|cand| self.wordp(cand))  {
            let prob = self.probability(c);
            if prob > max {
                max = prob;
                // correction = *c;
                correction = c.clone() ;
            }
        }
        correction
    }


    /// Generates slices of a word taken at increasing indicies
    fn slices(&self, word: String) -> Vec<(String, String)> {
        let mut slices: Vec<(String, String)> = Vec::new();
        for i in 0..word.len()+1 { // range is not inclusive
            slices.push((word[..i].to_string(),
                         word[i..].to_string()));
        }
        slices
    }
    
    /// Builds a list of all the candidate words for 
    /// the "correct" function to parse 
    fn candidates(&self, word:String)  -> Vec<String> {
        let mut cand: Vec<String> = Vec::new();

        // find where we need to start changing 
        let mut level = 0;

        for (left, right) in self.slices(word) {
            if right != "" {
                cand.push(left.clone() + &right[1..]); // deletion
            }
            if right.len() > 1 {
                cand.push(left.clone() + &right[1..2]
                          + &right[0..1] + &right[2..]); // transpose
            }

            // this where the Trie comes in handy 
            let choices = match find_chars_in_level(&self.cache, level, &left) {
            	Some(val) => val,
            	_		  => vec![],
            };

            for choice in choices {
            	 if right != "" {
                    cand.push(left.clone() + &choice.to_string() + &right[1..]); // replace   
                }
            	cand.push(left.clone() + &choice.to_string() + &right); // insert
            }
            level += 1;
        }
        cand
    }

    
    /// Checks to see if the word is present in the dictionary 
    pub fn wordp(&self, word: &String) -> bool {
        match self.counts.get(word) {
            Some(_) => true,
            None => false 
        }
    }
 }

fn add_to_trie(mut trie: &mut Trie, word: &str) { 

	let mut more_chilren = false;
	let mut child_index = 0;

	//recursion
	let mut chars = word.chars();

	if let Some(letter) = chars.next() {
		
		let letter_index = find_root(&mut trie, letter);

		if let Some(l) = chars.next() {
			more_chilren = true;
			child_index = find_root(&mut trie[letter_index].children, l);
		}
		if more_chilren{
			add_to_trie(&mut trie[letter_index].children[child_index].children, &word[2..])
		}
	}
}

fn find_chars_in_level(trie: &Trie, level: usize, word: &str) -> Option<Vec<char>> {
	let mut curr_level = 0;
	let mut curr_trie: &Trie = trie;

	
	//otherwise we can utilize the Trie (cache) data structure
	while curr_level < level {
		let mut cont = false;
		for node in curr_trie {
			if node.letter == word.chars().nth(curr_level).unwrap_or('~') {
				curr_trie = &node.children;
				curr_level += 1;
				cont = true;
			}
		}
		if !cont { break; }
	}
	let mut chars = vec![];
	for node in curr_trie {
		chars.push(node.letter.clone());
	}

	Some(chars)
}

fn find_root(trie: &mut Trie, letter: char) -> usize {
	for x in 0..trie.len() {
		if trie[x].letter == letter { return x }
	}
	// if we get here, it is a new root node 
	let root_node = Node::new(letter);

	trie.push(root_node);
	trie.len() - 1
}


#[cfg(test)] mod dict_tests {
    use super::SpellChecker;
    use std::io::Cursor;
    
    #[test] fn assert_counts() {
        let res = SpellChecker::new(Cursor::new(
            "hello hello great world \n the world"));
        
        assert_eq!(res.counts["hello"], 2);
        assert_eq!(res.counts["great"], 1);
        assert_eq!(res.counts["world"], 2);
        assert_eq!(res.counts["the"], 1);
        assert_eq!(res.sum, 6.);
    }

    #[test] fn assert_probability() {
        let res = SpellChecker::new(Cursor::new(
            "hello hello great world \n the world"));
        
        assert_eq!(res.probability(&String::from("hello")), 2./6.);
        assert_eq!(res.probability(&String::from("dog")), 0.);
    }

    #[test] fn assert_correct_choice() {
        let mut res = SpellChecker::new(Cursor::new("hello world hello word hello world"));

        assert_eq!(res.probability(&String::from("world")), 2./6.);
        assert_eq!(res.probability(&String::from("word")), 1./6.);
        assert_eq!(res.correct(String::from("wor")), "world");

        }

    #[test] fn assert_slices() {
        let res = SpellChecker::new(Cursor::new(""));
        let slices = res.slices(String::from("cat"));
        assert_eq!(slices[0].0, "");
        assert_eq!(slices[0].1, "cat");
        assert_eq!(slices[1].0, "c");
        assert_eq!(slices[1].1, "at");
        assert_eq!(slices[2].0, "ca");
        assert_eq!(slices[2].1, "t");
        assert_eq!(slices[3].0, "cat");
        assert_eq!(slices[3].1, "");
    }

    #[test] fn assert_candidates() {
        let res = SpellChecker::new(Cursor::new(""));
        let cand = res.candidates(String::from("cat"));
        println!("candidates {:?}", cand);
        assert_eq!(cand.len(), 5); // this should only be the transpose and delete ones 
    }
    
    #[test] fn assert_wordp() {
        let res = SpellChecker::new(Cursor::new(
            "hello hello great world"));
        
        assert_eq!(res.wordp(&String::from("worl")), false);
        assert_eq!(res.wordp(&String::from("hello")), true);
    }

    #[test] fn assert_correct() {
        let mut res = SpellChecker::new(Cursor::new(
            "hello hello great world"));
        
        assert_eq!(res.correct(String::from("worl")), "world");
        assert_eq!(res.correct(String::from("hell")), "hello");
    }
}
