use std::io;
use std::io::Write;


// \x1b[1;31m\x1b[31m => Red Bold
// \x1b[1;31m\x1b[33m => Yellow Bold
// \x1b[1;31m\x1b[34m => Blue Bold
// \x1b[2m => Faint
// \x1B[22m\x1b[39m => Clear

/// # bitwise operations and bit-packing in The Rust Programming Language.
/// This little program is a practice to dominate the bitwise operators
/// in bit-packing, in fact it didn't have a real advantage because it only
/// lives in DRAM and all disapear when it finishes.
///
/// This tecnique is used to store values in limited storage, compression
/// and transmission protocols.

fn main()
{
    let mut answers: u8 = 0;
    println!("The following questions are \x1b[1;31m\x1b[34m[y]es \x1B[22m\x1b[39mor \x1b[1;31m\x1b[31m[n]ot \x1B[22m\x1b[39manswers:");

    let mut i: u8 = 0;
    while i < 8
    {
        answers = write(answers, i, interviewer(i));
        i += 1;
    }

    print!("\n");

    print!("Do you wanna change someone of your responces? \x1b[2m(default: n)\x1B[22m \x1b[1;31m\x1b[33m>\x1B[22m\x1b[39m ");

    let mut wanna_redo: String = Default::default();

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut wanna_redo)
        .expect("Failed to read your choice!\n");

    let wanna_redo: String = wanna_redo
        .trim()
        .parse()
        .expect("Failed to sanitize input!");

    if wanna_redo == "y"
    {
        answers = change_answer(answers);
    }else if wanna_redo == "n" || wanna_redo == "" {
        // Do nothing.
    }else{
        println!("Not a valid responce.");
    }

    print!("\nSaving...");

    responce(answers);
}

/// ## `fn change_answer()`
/// This function is maded to give use to the `fn flip()` function
/// , it ask the user to give the number of question that want to
/// change in the registered answer calling to the flip function,
/// it goes into a `loop`, giving the possibility to change the 
/// answers, whatever the time's the user want to.

/// There, the `answr_nmbr` is re-asigned after the input, that's to
/// sanitize the given input.
/// Then, it compares `answr_nmbr` to "n", if the user want to quit
/// the change_answer function it return the `answers_registry` that
/// could be modified in previous iterations of the `loop`. If that
/// condition isn't true it converts `answr_nmbr` to an u8 re-assigning
/// it, then check if is minus or ecual than 8, if it's true does.


fn change_answer(mut answers_registry: u8) -> u8
{
    print!("Whitch one of your answers you wanna change? \x1b[1;31m\x1b[33m>\x1B[22m\x1b[39m ");

    let mut tolerance: u8 = 0;

    loop
    {
        let mut answr_nmbr: String = String::new();

        let _ = io::stdout().flush();

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

            match foot_shoot(answr_nmbr)
            {
                Ok(mut number) => {
                    if number <= 8
                    {
                        number -= 1;
                        answers_registry = flip(answers_registry, number);
                    }else{
                        println!("\x1b[1;31m\x1b[31mError:\x1B[22m\x1b[39m Index out of bounds!");
                        if tolerance < 3 {println!("Try again!");}else{return answers_registry;}
                        tolerance += 1;
                    }
                },
                Err(..) => {
                     println!("That didn't look like a valid responce!");
                     return answers_registry;
                }
            }
        }
        println!("\x1b[1;31m\x1b[33m=>\x1B[22m\x1b[39m {}", list(answers_registry));
        print!("Do you wanna change other answer?, \x1b[2mnumber/(default: n)\x1B[22m \x1b[1;31m\x1b[33m>\x1B[22m\x1b[39m ");
    }
}

fn foot_shoot(answr_nmbr: String) -> std::result::Result<u8, std::num::ParseIntError>
{
    let answr_nmbr: u8 = answr_nmbr
        .trim()
        .parse()?;
    return Ok(answr_nmbr);
}

/// ## `fn comparison()`
/// This function prints the actual space that storing the values
/// of the answers could be using if it, for example if uses a Vec<Bool>
///
/// It works printing seven "0" and fetching the bit in the possition
/// specified by the iterator of the `while` loop and printing it,

fn comparison(word: u8)
{
    println!("\nYou could be using 8 bytes of storage:");

    let mut i = 0;
    while i < 8
    {
        print!("[0000000");

        if read(word, i) == 1
        {
            println!("\x1b[1;31m\x1b[34m1\x1B[22m\x1b[39m] --> \x1b[1;31m\x1b[34m(true)\x1B[22m\x1b[39m");
        }else{
            println!("\x1b[1;31m\x1b[31m0\x1B[22m\x1b[39m] --> \x1b[1;31m\x1b[31m(false)\x1B[22m\x1b[39m");
        }
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
            1 => response += "\x1b[1;31m\x1b[34m1\x1B[22m\x1b[39m",
            0 => response += "\x1b[1;31m\x1b[31m0\x1B[22m\x1b[39m",
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

    print!("\x1b[1;31m\x1b[33m0{}.\x1B[22m\x1b[39m {}? \x1b[1;31m\x1b[33m>\x1B[22m\x1b[39m ", index + 1, questions_db[index as usize]);

    let mut answer: String = Default::default();

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer!\n");

    let answer: String = answer
        .trim()
        .parse()
        .expect("Failed to sanitize input!");

    return if answer == "y" { true }else if answer == "n" { false }else{ println!("\x1b[1;31m\x1b[33m=>\x1B[22m\x1b[39m Not such a valid response!, again!"); interviewer(index) }
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
