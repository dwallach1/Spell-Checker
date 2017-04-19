/**!
*
*	HOMEWORK 2 -- SPELL CHECKER by David Wallach & Matt George 
*	

*   
*	HOW IT WORKS:
*        
*           cargo run path_to_file < input.txt
*                
*            or 
*
*            cargo run path_to_file
*
*   where path_to_file is a file containg a list of words. Each word in this file
*   is parsed and stored as the programs dictionary. This dictionary is used to correct the words  
*   that appear in the input.txt file. 
*
*   INPUT:
*
*   path_to_file: hello world hello word hello world
*
*   input.txt: 
*           hello
*           hell
*           word
*           worldl
*           wor
*           wo
*           w
*
*
*
*   OUTPUT:
*
*          hello
*          hell, hello
*          word 
*          worldl, world
*          wor, world
*          wo, word
*          w, -
*
*	TRIE STRUCTURE OPTIMIZATION
*
*	From standard input, we can devleop a Trie with the root nodes being the starting letter 
*	two words with the same starting letter will have the same root node. 
*					
*			   Node_list = [w, ......., h]   			
*						   	|		    |
*						  	o 		    e
*							|		   / \
*							r		  y	  l
*						   / \			   \
*						  l	  d 			l
*						 / 					 \
*					    d 					  o 
*
*
*	Due to time constraints, we were not able to use this aspect to optimize the entire process. We still build a hashmap 
* 	with all the definitions and their associated frequencies, but the Trie allows us to minimize the number of possible
*	words when making word pairings
*
*   ASSUMPTIONS:
*
*   We did not account for punctuation or the case of letters in a word
*   
*           hello != Hello != Hello.
*   
*   therefore, each of these words would be stored in different entries in the dictionary 
*/

mod spell_checker;

use std::io::{BufReader, BufRead, stdin};
use std::env;
use std::path::Path;
use std::fs::File;


fn main() {
    let mut checker = load(env::args().nth(1).unwrap());
    let input = BufReader::new(stdin());
    
    for it_word in input.lines() { // specification is 1 word-per-line
        let word = it_word.unwrap();
        match checker.wordp(&word) { 
            true => println!("{}", word), // if word is a valid word according to dictionary, then return it
            false => println!("{}, {}", word, checker.correct(word.clone())), // otherwise, try to find a replacement 
        }
    }
}


/// Creates a spellchecker based off of the corpus
/// specified by the first argument.
fn load(fname: String) -> spell_checker::SpellChecker {
    // let fname = env::args().nth(1).unwrap();
    
    let path = Path::new(&fname);
    let display = path.display();
    println!("\nTrying to open file {}...", display);
    let file = match File::open(&path) {
        Ok(file) => file,
        _        => panic!("Could not open {}!", display),
    };
    println!("success\n");
    spell_checker::SpellChecker::new(file)
}


#[cfg(test)]
mod file_loading_tests {
    use super::load;

    #[test] 
    #[should_panic]
    fn assert_panic_bad_file() {
        load(String::from("text/fake.txt"));
    }

    #[test]
    #[allow(unused_variables)]
    fn assert_load() {
        let spell_checker = load(String::from("text/corpus.txt"));
        // if we get here with no panic ==> successfuly loaded file
        assert_eq!(0, 0);
    }
}

