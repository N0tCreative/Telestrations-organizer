use std::io;


fn main() {
    //block contains user input
    fn input() ->u8{
        println!("Enter the number of people playing broken picture telephone:");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input.truncate(user_input.len()-2);
        let parse =user_input.parse::<u8>();
        if parse.is_err(){
            println!("Error:{:?}", parse);
            return 0;
        }
        let num_people: u8 =parse.unwrap();
        return num_people;
    }

    let num_people:u8 =input();
    if num_people ==0 {
        return;
    }
    
    display_send_order(num_people);

}

//displays the order that the book could be sent in
fn display_send_order(num_people: u8){
    fn display_head(num_people:u8){
        let mut i:u8 =1;
        print!("Start \t");
        while i <num_people{
            print!("Round {} \t",i);
            i+=1;
        }
        println!("");
    }


    display_head(num_people);
    generate_all_perms(num_people);

}

fn generate_all_perms(num_people: u8){
    fn generate_first_perm (perm: &mut Vec<Vec<u8>>){
        let mut i:usize =0;
        while i<perm.len() {
            perm[i][0] =i as u8;
            i+=1;
        }
        i=0;
        print!("[");
        while i<perm.len() {
            print!("{} ", perm[i][0]);
            i+=1;
        }
        println!("]");
    }
    
    //each row is a round and each column is a person, each number is the book number
    let mut perm = vec![vec![0u8;num_people.into()]; num_people.into()];
    generate_first_perm(&mut perm);



}
