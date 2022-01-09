use std::io;
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    //block contains user input
    fn input() ->i8{
        println!("Enter the number of people playing broken picture telephone:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input.truncate(user_input.len()-2);
        let parse =user_input.parse::<i8>();
        if parse.is_err(){
            println!("Error:{:?}", parse);
            return 0;
        }
        let num_people: i8 =parse.unwrap();
        return num_people;
    }

    let num_people:i8 =input();
    if num_people <=0 {
        return;
    }
    
    //tests all permutations of the book, people, and round combination to find an optimal solution (either many or none exist, in testing if number is odd no optimal solution exists)
    generate_all_perms(num_people);
}

//displays the order that the book could be sent in
fn display_send_order(perm: Vec<Vec<i8>>){
    fn display_head(num_people:i8){
        let mut i:i8 =0;
        print!("\t");
        while i <num_people{
            print!("Person #{}\t",i);
            i+=1;
        }
        println!("");
    }
    fn display_body(perm: Vec<Vec<i8>>){
        let mut round_num:u8 =0;
        for round in perm {
            print!("Round {}\t", round_num);
            round_num +=1;
            for person in round {
                print!("{}\t\t", person); //print the book that each person should have at this round
            }
            //println!("]");
            println!("");
        }
    }
    display_head(perm.len() as i8);
    display_body(perm);
}

//using multithreading it generates all possible games that could be made with num_people (no person gets the same book twice and each book appears once per round)
//if a solution is found that involves each person not sending a book to the same person twice it is displayed and ends that thread (this is considered an optimal solution)
fn generate_all_perms(num_people: i8){
    //first round always starts with person x starting with book x (as that is what defines what the book number is)
    fn generate_first_perm (perm: &mut Vec<Vec<i8>>){
        let mut i:usize =0;
        while i<perm.len() {
            perm[0][i] =i as i8;
            i+=1;
        }
    }

    //assigns each person a book in a loop then recursively goes through each person then once its finished generating the round it recursively generates the next round
    //this is very inefficient but i dont think there is another way to do this
    fn generate_round (perm: &mut Vec<Vec<i8>>, round: i8, person: i8, lock:Arc<Mutex<i8>>)->bool {
        let mut valid_option =false;
        //if you generate a valid round try to generate the next one
        if person >= perm.len() as i8{
            //println!("got to the end of round {}", round);
            //if all rounds are generated then return
            if round < (perm.len()-1) as i8 {
                valid_option =generate_round(perm, round +1, 0, lock);

            } else {
                //reached a valid case
                valid_option =is_optimal(perm.to_vec());
                if valid_option {
                    let _l =lock.lock().unwrap();
                    println!("\n~~~~~~~~~~~~~~AN OPTIMAL SOLUTION!~~~~~~~~~~~~~~");
                    display_send_order(perm.to_vec());
                }
                
            }
            return valid_option;
        }
        
        //test every possible book option for this person given the previous choices in the round
        for book  in 0..perm.len() as i8 {
            //println!("testing {:?} with person{} having book{}", perm[round as usize], person, book);
            let mut valid_book:bool =true;
            //test if anyone else this round has this book already
            for test_book in 0..person {
                if perm[round as usize][test_book as usize] == book {
                    //println!("invalid, person{} has this book already", test_book);
                    valid_book = false;
                    //break;
                }
            }
            //test if this person already had this book in a prior round
            for test_round in 0..round {
                if perm[test_round as usize][person as usize] == book {
                    //println!("invalid, at round{} they already had this book", test_round);
                    valid_book = false;
                    //break;
                }
            }
            //if this person could have this book then try to give a book to the next person
            if valid_book {
                //println!("valid");
                perm[round as usize] [person as usize] = book;

                //Arcs cant be passed mutably so for each recursion it needs to be cloned
                let lock = Arc::clone(&lock);
                valid_option =generate_round(perm, round, person +1, lock);
                //if this solution is optimal then ignore all subsiquent options
                if valid_option {
                    break;
                }
            }
        }

        return valid_option;
    }

    //sets the book to start and end on (basically generate_round but evenly divides the work for multithreading)
    fn generate_first_round (perm: &mut Vec<Vec<i8>>, start_book: i8, end_book:i8, lock:Arc<Mutex<i8>>)->bool {
        let mut valid_option =false;
        println!("start{} end{}",start_book, end_book);
        //test every possible book option for this person given the previous choices in the round
        for book  in start_book..end_book {
            //println!("testing {:?} with person{} having book{}", perm[round as usize], person, book);
        
            //println!("valid");
            perm[1] [0] = book;
            //Arcs cant be passed mutably so for each recursion it needs to be cloned
            let lock = Arc::clone(&lock);
            valid_option =generate_round(perm, 1, 1, lock);
            //if this solution is optimal then ignore all subsiquent options
            if valid_option {
                break;
            }
        }

        return valid_option;
    }

    //checks if the solution generated involves sending to a different person each round
    fn is_optimal (perm: Vec<Vec<i8>>)->bool {
        let mut optimal = true;
        let mut sent_to = vec![vec![0u8;perm.len()];perm.len()];
        //for each round except the last round
        for i in 0..(perm.len()-1) as i8 {
            //for each book in that round
            for j in 0..perm.len() as i8 {
                let mut sender:i8 =0;
                let mut reciever:i8 =0;
                //find the person with that book
                while sender <perm.len() as i8 {
                    if perm[i as usize][sender as usize] ==j {
                        break;
                    }
                    sender+=1;
                }
                //find the person with that book in the next round
                while reciever <perm.len() as i8 {
                    if perm[(i+1) as usize][reciever as usize] ==j {
                        break;
                    }
                    reciever+=1;
                }
                if sent_to[sender as usize][reciever as usize] ==1 {
                    optimal = false;
                    return optimal;
                } else {
                    sent_to[sender as usize][reciever as usize] =1;
                }
                
            }
        }
        return optimal;
    }
    
    //i should make this a 2d vector so that this could be looped but im not sure if the borrow checker 
    //will allow multiple threads to access different parts of the same vector at the same time and this is just a proof of concept

    //each row is a round and each column is a person, each number is the book number
    let num_threads =4;
    let mut perm1 = vec![vec![-1i8;num_people as usize]; num_people as usize];
    let mut perm2 = vec![vec![-1i8;num_people as usize]; num_people as usize];
    let mut perm3 = vec![vec![-1i8;num_people as usize]; num_people as usize];
    let mut perm4 = vec![vec![-1i8;num_people as usize]; num_people as usize];
    //minimum number of books per thread is the number of books left to choose (0 has already been chosen) divided by the number of threads (4)
    let min_books_per_thread = (num_people -1) / num_threads;
    //if number of books left is not evenly divis by number of threads then spread out the remainder to as many threads as possible
    let mut remainder = (num_people-1) % num_threads;
    let mut start =1;
    let mut end;
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock1);
    let lock3 = Arc::clone(&lock1);
    let lock4 = Arc::clone(&lock1);


    //set all initial values for what book each person starts with
    generate_first_perm(&mut perm1);
    generate_first_perm(&mut perm2);
    generate_first_perm(&mut perm3);
    generate_first_perm(&mut perm4);
    //first thread
    if remainder > 0{
        end = start + min_books_per_thread +1;
        remainder -=1;
    } else {
        end = start + min_books_per_thread;
    }
    let handle1 = thread::spawn(move || generate_first_round(&mut perm1, start, end, lock1));

    //sencond thread
    start = end;
    if remainder > 0{
        end = start + min_books_per_thread +1;
        remainder -=1;
    } else {
        end = start + min_books_per_thread;
    }
    let handle2 = thread::spawn(move || generate_first_round(&mut perm2, start, end, lock2));

    //third thread
    start = end;
    if remainder > 0{
        end = start + min_books_per_thread +1;
    } else {
        end = start + min_books_per_thread;
    }
    let handle3 = thread::spawn(move || generate_first_round(&mut perm3, start, end, lock3));

    //fourth thread (no need to check for a remainder because if there still was one then it would be divisiable by the number of threads so there wouldnt be one in the first place)
    start = end;
    end = start + min_books_per_thread;
    let handle4 = thread::spawn(move || generate_first_round(&mut perm4, start, end, lock4));

    //wait for all threads to end then ends the main thread
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
}
