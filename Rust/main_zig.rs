use std::{io, io::Write};


// \x1b[1;31m\x1b[31m => Red Bold
// \x1b[1;31m\x1b[33m => Yellow Bold
// \x1b[1;31m\x1b[34m => Blue Bold
// \x1b[1;31m\x1b[32m => Green Bold
// \x1b[2m => Faint
// \x1B[22m\x1b[39m => Clear

fn main()
{
    let questions_db = [
        "A pointer is a reference in memory to an actual value",
        "Is allocating in the heap faster than pushing to the stack",
        "Function calling could be maded with JMP instructions",
        "This program is runing from your hard storage",
        "Concurrency allow CPU's to manage multiple threads",
        "A thread is like another processor more",
        "A compiler translates a program into a series of CPU instructions each time it runs",
        "Is a Interpreted language slower than compiled ones" 
    ];

    let feedback = [
        "A pointer is a variable that stores the direction in memory of the data it refers.",
        "Searching through, for freed space were the data fits\n    is more complex and it bring more CPU instructions than\n    storing a value and a pointer to where the data starts.",
        "At low level, function calling and returning, is maded\n    from JMP instructions, same as loops and conditionals.\n    Or inline functions replace the code where is called with the entire function.",
        "This program run from intructions loaded from hard disk into DRAM,\n    all the program is loaded while it runs, when finishes is cleaned.",
        "Concurrency allow a CPU to have many proceses running concurrently\n    so it gives the ilusion that are runnig at the same time.",
        "A thread is a CPU core that could run in paralel with many others.",
        "A compiler does translate statements into CPU instructions, the\n    resultant is a binary file that could be runned multiple times.",
        "Interpreted languajes are shit."
    ];

    const REFERENCE: u8 = 0b10100101;
    let mut answers: u8 = 0;
    println!("\x1b[22mThe following questions are \x1b[1;31m\x1b[34m[t]rue \x1B[22m\x1b[39mor \x1b[1;31m\x1b[31m[f]alse \x1B[22m\x1b[39manswers:");

    let mut modality: String = Default::default();

    print!("How do you want the feedback for your responces? \x1b[1;31m\x1b[33m>\x1b[22m\x1b[39m \n \x1b[2m\x1b[33m[A] =>\x1b[39m At the end of the program.\n \x1b[33m[B] =>\x1b[39m After each responce.\n \x1b[22m\x1b[39m\x1B[s\x1B[3A\x1B[50C");

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut modality)
        .expect("Failed to read desition!");

    let mut modality: String = modality
        .trim()
        .parse()
        .expect("Failed to sanitize the input!");

    print!("\x1B[u\n");

    while modality != "A" && modality != "B"
    {
        eprint!("Press [A] or [B], or [C] to get the fuck out of here! \x1b[1;31m\x1b[33m>\x1b[22m\x1b[39m ");

        let mut modality_a: String = Default::default();

        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut modality_a)
            .expect("Failed to read desition!");

        let modality_a: String = modality_a
            .trim()
            .parse()
            .expect("Failed to sanitize the input!");
        if modality_a == "C"{
            panic!("It was your fault!");
        }else if modality_a == "A" ||  modality_a == "B"{
            modality = modality_a;
            print!("\n");
        }else{eprintln!("Try again!");}
    }

    if modality == "A"{
        let mut i: u8 = 0;
        while i < 8
        {
            answers = write(answers, i, interviewer(i, questions_db[i as usize].to_string()));
            i += 1;
        }
        println!("\nCorrection:");
        i = 0;
        while i < 8
        {
            println!("\n0{}. {}? > {}", i + 1, questions_db[i as usize], if read(answers, i) == 1{"true"}else{"false"} );
            ffeedback(i, answers, REFERENCE, feedback[i as usize].to_string());
            i += 1;
        }
    }else if modality == "B" {
        let mut i: u8 = 0;
        while i < 8
        {
            answers = write(answers, i, interviewer(i, questions_db[i as usize].to_string()));
            ffeedback(i, answers, REFERENCE, feedback[i as usize].to_string());
            i += 1;
        }
    }else{panic!("This is the panic! trighering");}

    print!("\n");


    responce(answers, REFERENCE);
}


fn ffeedback(index: u8, answer: u8, reference: u8, feedback: String)
{
    if read(answer, index) == read(reference, index) {
        println!("\x1b[1;31m\x1b[33m=> \x1b[32mCorrect!\x1B[22m\x1b[39m");
        println!("\x1b[1;31m\x1b[33m¿*? \x1b[32m{feedback}\x1B[22m\x1b[39m");
    }else{
        eprintln!("\x1b[1;31m\x1b[33m=> \x1b[31mWrong!\x1B[22m\x1b[39m");
         println!("\x1b[1;31m\x1b[33m¿*? \x1b[31m{feedback}\x1B[22m\x1b[39m");
    }
}

fn comparison(word: u8, reference: u8)
{
    println!("\nYou could be using 8 bytes of storage:");

    let mut i = 0;
    while i < 8
    {
        print!("[0000000");

        let readed: u8 = read(word, i);

        if readed == 1{
            print!("\x1b[1;31m\x1b[34m1\x1B[22m\x1b[39m] --> (\x1b[1;31m\x1b[34mtrue\x1B[22m\x1b[39m) ");
        }else{
            print!("\x1b[1;31m\x1b[31m0\x1B[22m\x1b[39m] --> (\x1b[1;31m\x1b[31mfalse\x1B[22m\x1b[39m)");
        }

        let ref_readed: u8 = read(reference, i);

        if readed == ref_readed {
            println!(" \x1b[1;31m\x1b[33m=> \x1b[32mCorrect!\x1B[22m\x1b[39m");
        } else {
            println!(" \x1b[1;31m\x1b[33m=> \x1b[31mWrong! \x1B[22m\x1b[39m({}\x1B[22m\x1b[39m)", if ref_readed == 1 {"\x1b[1;31m\x1b[34mtrue"} else {"\x1b[1;31m\x1b[31mfalse"});
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


fn count_c(answers: u8, reference: u8) -> u8
{
    let mut counter: u8 = 0;
    let mut i: u8 = 0;
    while i < 8
    {
        if read(reference, i) == read(answers, i){counter += 1;}else{}
        i += 1;
    }
    return counter;
}

fn responce(answers: u8, reference: u8)
{
    print!("\n");
    let count: u8 = count_c(answers, reference);
    if count < 5 {print!("\x1b[1;31m\x1b[31m");}else if count <= 8 {print!("\x1b[1;31m\x1b[32m");} else {panic!();}
    println!("You picked correctly {}/8 responces.\x1B[22m\x1b[39m", count);
    print!("\n");
    comparison(answers, reference);
    println!("\nInstead that's using: {}", list(answers));
    println!("                          |-> {answers} in decimal.\n");
}

fn interviewer(index: u8, question: String) -> bool
{

    print!("\n\x1b[1;31m\x1b[33m0{}.\x1B[22m\x1b[39m {}? \x1b[1;31m\x1b[33m>\x1B[22m\x1b[39m ", index + 1, question);

    let mut answer: String = Default::default();

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer!\n");

    let answer: String = answer
        .trim()
        .parse()
        .expect("Failed to sanitize input!");

    return if answer == "t" { true }else if answer == "f" { false }else{ println!("=> Not such a valid response!, again!"); interviewer(index, question) }
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

// fn flip(answers_registry: u8, possition: u8) -> u8 { answers_registry ^ (1 << (possition)) }
