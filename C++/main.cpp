#include <iostream>
#include <string>
#include <print>


// \x1b[1;31m\x1b[31m => Red Bold
// \x1b[1;31m\x1b[33m => Yellow Bold
// \x1b[1;31m\x1b[34m => Blue Bold
// \x1b[2m => Faint
// \x1B[22m\x1b[39m / \x1b[0m => Clear


void feedback(unsigned char answers_registry, unsigned char REFERENCE, unsigned char index);
unsigned char write(unsigned char answers_registry, unsigned char possition, bool digit);
unsigned char read(unsigned char answers_registry, unsigned char possition);
unsigned char flip(unsigned char answers_registry, unsigned char possition);
std::string list (unsigned char answers_registry);
bool interviewer(short index);
unsigned char change_answer(unsigned char answers_registry);
void comparison(unsigned char answers);
void responce(unsigned char answers);


int main()
{
	std::string feedback_mode;
	std::print("How do you want the feedback for your responses \x1b[1;31m\x1b[33m:\x1b[0m \n \x1b[1;31m\x1b[33m[A]\x1b[0m => After each response.\n \x1b[1;31m\x1b[33m[B]\x1b[0m => At the end of the program.\x1B[s\n\x1B[3A\x1B[50C");
	std::cin >> feedback_mode;

	std::println("\x1b[u");
	while (feedback_mode != "A" && feedback_mode != "B") {
		std::print("Press \x1b[33m[A]\x1b[0m or \x1b[33m[B]\x1b[0m, or \x1b[33m[C]\x1b[0m to \x1b[1;31m\x1b[31mget the fuck out\x1b[0m of here! \x1b[1;31m\x1b[33m>\x1b[0m ");
		std::cin >> feedback_mode;
		if (feedback_mode == "C") { return 1; } else {} // Ends the program if the user press "C".
	}

	std::println("The following questions are [y]es or [n]ot answers:");
	unsigned char answers = 0;
	const unsigned char reference = 0b01110000;

	if (feedback_mode == "A") {
		for(short i = 0; i < 8 ; i++)
		{
			answers = write(answers, i, (interviewer(i)));
			feedback(answers, reference, i);
		}
	} else if (feedback_mode == "B") {
		for(short i = 0; i < 8 ; i++)
		{
			answers = write(answers, i, (interviewer(i)));
		}
		feedback(answers, reference, 1);
	} else {}
/*
	std::print("\nDo you wanna change someone of your responses?(default: n) > ");
	std::string wanna_change;
	std::cin >> wanna_change;
	if (wanna_change == "y")
	{
		answers = change_answer(answers);
	}else if (wanna_change == "n" || wanna_change == "")
	{
		// Also doing nothing
	}else{std::println("Not a valid response!");}
	std::println("\nSaving...");
*/
	responce(answers);
	std::print("\n");
}

std::string questions_db[8] = {
	"When you compile a program it's binary could run in every machine",
	"Is Processing like a dialect of C++",
	"Is Assembler a kind of a representation of machine code rather than a language",
	"Do you like the Zig Programming Language",
	"Is DOS earlier than UNIX",
	"A WORD is 2 bytes",
	"Is the microprocessor 8085 16-bit arch",
	"A compiler only converts the source code in binary, in one step, that's all",
};

std::string feedback_db[8] = {
	"When a compiler convert you code in instructions it's probably gonna create architecture specific instructions, or even new instructions of your processor, so older processors or different architectures couldn't run it.",
	"The Processing lang., used in Arduino is based on C++.",
	"Assembler shows each one of the instructions in machine code (binary) in readable characters and words, so in practice, it has no abstraction layers.",
	"That's OK!",
	"The first UNIX implementation was created in 1969, and DOS in 1981.",
	"That's debatable.",
	"The Intel 8085 was the last one of 8-bit architecture Intel madded.",
	"The compiler actually perform 4th different steps, in order:\n 1- Preprocessing.\n 2- Compiling.\n 3- Assembling.\n 4- Linking.",
};


void feedback(unsigned char answers_registry, unsigned char REFERENCE, unsigned char index)
{
	if (read(answers_registry, index) == read(REFERENCE, index)) {
		std::println(" => Correct.");
	} else {
		std::println(" => Wrong.");
	}
	std::println("{}", feedback_db[index]);
}


unsigned char write(unsigned char answers_registry, unsigned char possition, bool digit)
{
	switch (digit)
	{
		case true: return (answers_registry | (1 << possition));
		case false: return (answers_registry & ~(1 << possition));
	}
}

unsigned char read(unsigned char answers_registry, unsigned char possition){return (answers_registry >> possition) & 1;}

unsigned char flip(unsigned char answers_registry, unsigned char possition){return (answers_registry ^ (1 << possition));}

std::string list (unsigned char answers_registry)
{
	std::string string_of_bits;
	for(short i = 7; i >= 0; i--)
	{
		unsigned char i_readed = read(answers_registry, i);
		if ( i_readed == 0)
		{
			string_of_bits += "\x1b[1;31m\x1b[31m0";
		}else if(i_readed == 1){
			string_of_bits += "\x1b[1;31m\x1b[34m1";
		}else{std::println("Error in list(), index {}", i);}
	}
	return (string_of_bits + "\x1b[0m");
}

bool interviewer(short index)
{
	std::string answer;
	std::print("\x1b[1;31m\x1b[33m0{}.\x1b[0m {}? \x1b[1;31m\x1b[33m>\x1b[0m ", (index + 1), questions_db[index]);
	std::cin >> answer;
	if (answer == "y")
	{
		return true;
	}else if (answer == "n")
	{
		return false;
	}else
	{
		std::println("That doesn't seen like a valid response, again!");
		return interviewer(index);
	}
}


unsigned char change_answer(unsigned char answers_registry)
{
	std::print("Witch one of your answers you wanna change? > ");
	while (true)
	{
		std::string answr;
		std::cin >> answr;
		if(answr == "n" || answr == "" )
		{
			return answers_registry;
		}else{
			unsigned char answr_nmbr = stoi(answr);
			if (answr_nmbr <= 8)
			{
				answr_nmbr -= 1;
				answers_registry = flip(answers_registry, answr_nmbr);
			}else{
				std::println("That didn't look like a valid response!");
				return answers_registry;
			}
		}
		std::println("{}", list(answers_registry));
		std::print("Do you want to change other response?, number/(default: n) > ");
	}
}


void comparison(unsigned char answers)
{
	std::println("You could be using 8 bytes of storage:");
	std::string i_readed;
	for(short i = 0; i < 8; i++)
	{
		std::print("[0000000");
		if (read(answers, i) == 1)
		{
			std::println("\x1b[1;31m\x1b[34m1\x1b[0m] --> (\x1b[1;31m\x1b[34mtrue\x1b[0m)");
		}else{
			std::println("\x1b[1;31m\x1b[31m0\x1b[0m] --> (\x1b[1;31m\x1b[31mfalse\x1b[0m)");
		}
	}
}

void responce(unsigned char answers)
{
	std::print("\n");
	comparison(answers);
	std::println("\nInstead you're using only one: {}", list(answers));
	std::println("				      |-> {} in decimal.", answers);
}
