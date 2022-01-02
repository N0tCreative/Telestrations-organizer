use std::io;

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

fn generate_all_perms(num_people: i8){
    //first round always starts with person x starting with book x (as that is what defines what the book number is)
    fn generate_first_perm (perm: &mut Vec<Vec<i8>>){
        let mut i:usize =0;
        while i<perm.len() {
            perm[0][i] =i as i8;
            i+=1;
        }
    }

    fn generate_round (perm: &mut Vec<Vec<i8>>, round: i8, person: i8)->bool {
        let mut valid_option =false;
        //if you generate a valid round try to generate the next one
        if person >= perm.len() as i8{
            //println!("got to the end of round {}", round);
            //if all rounds are generated then return
            if round < (perm.len()-1) as i8 {
                valid_option =generate_round(perm, round +1, 0);

            } else {
                //reached a valid case
                valid_option =is_optimal(perm.to_vec());
                if valid_option {

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

                valid_option =generate_round(perm, round, person +1);
                //if this solution is optimal then ignore all subsiquent options
                if valid_option {
                    break;
                }
            }
        }

        return valid_option;
    }

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
    
    //each row is a round and each column is a person, each number is the book number
    let mut perm = vec![vec![-1i8;num_people as usize]; num_people as usize];
    generate_first_perm(&mut perm);
    //println!("prior to generate round\n{:?}", perm);
    let sol_exists:bool =generate_round(&mut perm, 1, 0);
    if !sol_exists {
        println!("No optimal solution exists for this number of people");
    }



}
