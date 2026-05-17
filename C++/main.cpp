#include <iostream>
#include <string>
#include <print>

std::string ltrim(const std::string &);
std::string rtrim(const std::string &);


/*
int main()
{
    string n_temp;
    getline(cin, n_temp);

    int n = stoi(ltrim(rtrim(n_temp)));
*/

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
			string_of_bits += "0";
		}else if(i_readed == 1){
			string_of_bits += "1";
		}else{std::println("Error in list(), index {}", i);}
	}
	return string_of_bits;
}

bool interviewer(short index)
{
	std::string answer;
	std::string questions_db[8] = {
	"Does exist micro-controlers with 64 bits architecture",
	"Is Processing like a dialect of C++",
	"Is Assembler a kind of a representation of machine code rather than a language",
	"Do you like the Zig Programming Language",
	"Is DOS earlier than UNIX",
	"A WORD is 2 bytes",
	"Is the microcontroler 8085 of 16-bit arch",
	"Is the LM555 a multi-vibrator",
	};
	std::print("0{}. {}? > ", (index + 1), questions_db[index]);
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
	std::println("Whitch one of your answers you wanna change?");
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
				std::println("That didn't look like a valid responce!");
				return answers_registry;
			}
		}
		std::println("{}", list(answers_registry));
		std::println("Do you want to change other responce?, number/(default: n)");
	}
}


void comparison(unsigned char answers)
{
	std::println("You could be using 8 bytes of storage:");
	std::string i_readed;
	for(short i = 0; i < 8; i++)
	{
		std::print("0000000");
		if (read(answers, i) == 1)
		{
			std::println("1 --> (true)");
		}else{
			std::println("0 --> (false)");
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

int main()
{
	std::println("The following questions are [y]es or [n]ot answers:");
	unsigned char answers = 0;
	for(short i = 0; i < 8 ; i++)
	{
		answers = write(answers, i, (interviewer(i)));
	}
	std::println("Do you wanna change someone of your responces?(default: n)");
	std::string wanna_change;
	std::cin >> wanna_change;
	if (wanna_change == "y")
	{
		answers = change_answer(answers);
	}else if (wanna_change == "n" || wanna_change == "")
	{
		// Also doing nothing
	}else{std::println("Not a valid responce!");}
	std::println("Saving...");
	responce(answers);
}
