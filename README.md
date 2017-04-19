# Spell-Checker


   
	HOW IT WORKS:
        
           cargo run path_to_file < input.txt
                
            or 

            cargo run path_to_file

   where path_to_file is a file containg a list of words. Each word in this file
   is parsed and stored as the programs dictionary. This dictionary is used to correct the words  
   that appear in the input.txt file. 

   INPUT:

   path_to_file: hello world hello word hello world

   input.txt: 
           hello
           hell
           word
           worldl
           wor
           wo
           w



   OUTPUT:

          hello
          hell, hello
          word 
          worldl, world
          wor, world
          wo, word
          w, -

	TRIE STRUCTURE OPTIMIZATION

	From standard input, we can devleop a Trie with the root nodes being the starting letter 
	two words with the same starting letter will have the same root node. 
					
			   Node_list = [w, ......., h]   			
        						  |		        |
        						  o 		      e
        							|		       / \
        							r		     y	  l
        						 / \			       \
        						l	  d 			      l
        					 / 					          \
        					 d 					           o 




	Due to time constraints, we were not able to use this aspect to optimize the entire process. We still build a hashmap 
 	with all the definitions and their associated frequencies, but the Trie allows us to minimize the number of possible
	words when making word pairings

   ASSUMPTIONS:

   We did not account for punctuation or the case of letters in a word
   
           hello != Hello != Hello.
   
   therefore, each of these words would be stored in different entries in the dictionary 
   