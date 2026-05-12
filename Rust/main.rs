use std::io;
// use std::string::String;


fn main()
{
    let mut answers: u8 = 0;
    println!("The following questions are [y]es or [n]ot answers:");

    answers = if interviewer(0){set(answers, 0)}else{clear(answers, 0)};

    answers = if interviewer(1){set(answers, 1)}else{clear(answers, 1)};

    answers = if interviewer(2){set(answers, 2)}else{clear(answers, 2)};

    answers = if interviewer(3){set(answers, 3)}else{clear(answers, 3)};

    answers = if interviewer(4){set(answers, 4)}else{clear(answers, 4)};

    answers = if interviewer(5){set(answers, 5)}else{clear(answers, 5)}

    answers = if interviewer(6){set(answers, 6)}else{clear(answers, 6)};

    answers = if interviewer(7){set(answers, 7)}else{clear(answers, 7)};


    // -----------------------------------------
    let answers_string: String = list(answers);

    responce(answers);

    print!("\n");
    comparison(answers);
    println!("\nInstead that's using: {}", answers_string);
    println!("                          |-> {answers} in decimal.\n");
}


fn comparison(word: u8)
{
    println!("You could be using 8 bytes of storage:");

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
    if read(answers, 6) == 1
    {
        println!("- Sorry, but you are in the wrong place!");
        if read(answers, 0) == 1
        {
            println!("- If so, do: \"$ nvim ./main.rs\" in you terminal emulator, there is the answer!");
        }else{
            println!("- If so, do: \"$ cat ./main.rs\" in you terminal emulator, there is the answer!");
        }
        if read(answers, 4) != 1
        {
            println!("\n- Oh!, and if you don't have one of these, maybe the lesson is not for you!");
        }else{
        }
    }
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

    answer = answer
        .trim()
        .parse()
        .expect("error[E001]");

    if answer == "y"
    {
        return true;
    }else if answer == "n"
    {
        return false;
    }else{
        println!("Not such a valid response!, again!");
        return interviewer(index);
    }
}

fn set(answers_registry: u8, possition: u8) -> u8 // Change the bit in
                                                  // the possition
                                                  // specified to "1".
{
    answers_registry | (1 << possition)
}

fn clear(answers_registry: u8, possition: u8) -> u8 // Change the bit in
                                                    // the possition 
                                                    // specified to "0".
{
    answers_registry & !(1 << possition)
}

fn read(answers_registry: u8, possition: u8) -> u8 // Read the bit in the 
                                                   // possition specified.
{
    return (answers_registry >> possition) & 1;
}


/*
fn flip(answers_registry: u8, possition: u8) -> u8 {
    answers_registry ^ (1 << (possition -1))
}
/*
|-> Change the bit 
 in the possition
 specified to the
 opposite.
*/*/

