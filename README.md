# Telestrations-organizer
with n number of people broken picture telephone involves each person getting a book, writting something in it, 
the next person has to get the book must draw what was written, the person after that must write what was drawn etc 
until there have been n rounds (each person has contributed once to each book). 
The first thing this should find is all sets of paths that would result in each person getting each book exactly once. 
the second thing this sould find is if its possible for each person to hand their current book to a different person each time.

essentially (1)generates every possible permutation of an array such that no number is in the same position in the array (this was what the program was originally for)
then (2)checks that the permutations dont move any numbers from one position to another more than once (this request was later added as the results from the first were trivial)
(ie the number in position 1 in round 1 is in position 2 in round 2 and the number in position 1 in round 3 is in position 2 in round 4)

this algorithm is very inefficiently designed and to make it faster adherence to the 2nd criteria should be checked while the permutations are being created instead of after, there are unnecessary loops for checking if a book has been used this round, and using permutations makes this O(!n) and not usable for more than 7 people for odd numbers of players, even numbered groups of people seem to be solved rather quickly
note that as far as i can tell there are no solutions for both criteria with an odd number of players
