use std::io;


fn main()
{
    let mut answers: u8 = 0;
    println!("The following questions are [y]es or [n]ot answers:");
    
    if interviewer(0) // Do you use NeoVim?
    {
        answers = set(answers, 0);
    }else{
        answers = clear(answers, 0);
    }

    if interviewer(1) // Do you like programming?
    {
        answers = set(answers, 1);
    }else{
        answers = clear(answers, 1);
    }
    
    if interviewer(2) // Do you watch Core Dumped?
    {
        answers = set(answers, 2);
    }else{
        answers = clear(answers, 2);
    }
    
    if interviewer(3) // Is Rust in the top 3° better langs?
    {
        answers = set(answers, 3);
    }else{
        answers = clear(answers, 3);
    }
    
    if interviewer(4) // Are you Derek?
    {
        answers = set(answers, 4);
    }else{
        answers = clear(answers, 4);
    }
    
    if interviewer(5) // Is Linux the best OS?
    {
        answers = set(answers, 5);
    }else{
        while !interviewer(5)
        {
                continue;
        }

        answers = set(answers, 5);
    }
    
    if interviewer(6) // Do you wanna learn about bitpacking?
    {
        answers = set(answers, 6);
    }else{
        answers = clear(answers, 6);
    }




    // -----------------------------------------

    responce(answers);

    println!("\nResponses: {}", list(answers));
    println!("Or {answers} in decimal.");
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

fn interviewer(index: usize) -> bool
{
    let questions_db = [
    "Do you use NeoVim",
    "Do you like programming",
    "Do you watch Core Dumped",
    "Is Rust in the top 3° better langs",
    "Are you Derek",
    "Is Linux the best OS",
    "Do you wanna learn about bitpacking"
    ];

    println!("0{}. {}?", index + 1, questions_db[index]);

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

