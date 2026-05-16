use std::io;
// use std::string::String;


fn main()
{
    let mut answers: u8 = 0;
    println!("The following questions are [y]es or [n]ot answers:");

    let mut i: u8 = 0;
    while i < 8
    {
        answers = write(answers, i, interviewer(i));
        i += 1;
    }

    print!("\n");
    println!("Do you wanna change someone of your responces? (default: n)");

    let mut wanna_redo: String = Default::default();

    io::stdin()
        .read_line(&mut wanna_redo)
        .expect("Failed to read your choice!\n");

    let wanna_redo: String = wanna_redo
        .trim()
        .parse()
        .expect("Failed to sanitize input!");

    if wanna_redo == "y"
    {
        answers = redo_answers(answers);
    }else if wanna_redo == "n"{
        // Do nothing.
    }else{
        println!("Not a valid responce.");
    }

    print!("Saving...");

    responce(answers);
}

/// ## `fn redo_answers()`
/// This function is maded to give use to the `fn flip()` function
/// it does ask the user to give the number of question that wanna
/// do again and stores his value in a 
fn redo_answers(mut answers_registry: u8) -> u8
{
    println!("Whitch one of your answers you wanna change?");

    loop
    {
        let mut answr_nmbr: String = String::new();

        io::stdin()
            .read_line(&mut answr_nmbr)
            .expect("Failed to read question to change!");

        let answr_nmbr: String = answr_nmbr
            .trim()
            .parse()
            .expect("Failed to sanitize input!");

        if answr_nmbr == "n" || answr_nmbr == ""
        {
            return answers_registry;
        }else{
            let mut answr_nmbr: u8 = answr_nmbr
                .trim()
                .parse()
                .expect("Failed to convert to number!");

            if answr_nmbr <= 8
            {
                answr_nmbr -= 1;
                answers_registry = flip(answers_registry, answr_nmbr);
            }else{
                return answers_registry;
            }
        }
        println!("{}", list(answers_registry));
        println!("Do you wanna change other answer?, number/(default: n)");
    }
}

fn comparison(word: u8)
{
    println!("\nYou could be using 8 bytes of storage:");

    let mut i = 0;
    while i < 8
    {
        print!("0000000");

        println!("{}", read(word, i));

        i += 1;
    }
}

fn list(answers: u8) -> String
{
    let mut index: u8 = 0;
    let mut response: String = Default::default();

    while index < 8
    {
        match read(answers, 7 - index)
        {
            1 => response += "1",
            0 => response += "0",
            _other => panic!("Not possible"),
        }
        index += 1;
    }
    return response;
}

fn responce(answers: u8)
{
    print!("\n");
    comparison(answers);
    println!("\nInstead that's using: {}", list(answers));
    println!("                          |-> {answers} in decimal.\n");
    print!("\n");
}

fn interviewer(index: u8) -> bool
{
    let questions_db = [
    "Do you use NeoVim",
    "Do you like programming",
    "Do you watch Core Dumped",
    "Is Rust in the top 3° better langs",
    "Are you Derek",
    "Is Linux the best OS",
    "Do you think bit-packing is worth today",
    "Do you wanna learn about bitpacking"
    ];

    println!("0{}. {}?", index + 1, questions_db[index as usize]);

    let mut answer: String = Default::default();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer!\n");

    let answer: String = answer
        .trim()
        .parse()
        .expect("Failed to sanitize input!");

    return if answer == "y" { true }else if answer == "n" { false }else{ println!("Not such a valid response!, again!"); interviewer(index) }
}

fn write(answers_registry: u8, possition: u8, digit:/*--> digit indicates if it puts a 1 or 0 ->*/ bool) -> u8
{
    match digit
    {
        true => answers_registry | (1 << possition),
        false => answers_registry & !(1 << possition),
    }
}

fn read(answers_registry: u8, possition: u8) -> u8 { (answers_registry >> possition) & 1 }

fn flip(answers_registry: u8, possition: u8) -> u8 { answers_registry ^ (1 << (possition)) }
