#include <iostream>
#include <string>
#include <print>

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


unsigned char write(unsigned char word, unsigned char possition, bool digit)
{
	switch (digit)
	{
		case true: return (word | (1 << possition));
		case false: return (word & ~(1 << possition));
	}
}

unsigned char read(unsigned char word, unsigned char possition){return (word >> possition) & 1;}

// <--<--<--<--

std::string list (unsigned char word)
{
	std::string string_of_bits;
	for(short i = 7; i >= 0; i--)
	{
		unsigned char i_readed = read(word, i);
		if ( i_readed == 0)
		{
			string_of_bits += "0";
		}else if(i_readed == 1){
			string_of_bits += "1";
		}else{std::println("Error in list(), index {}", i);}
	
/*
		switch (read(word, i))
		{
			case 0: string_of_bits += "0";
			case 1: string_of_bits += "1";
			default: std::println("Error in list(), index {}", i);
		};
*/
//		string_of_bits += read(word, i);
	}
	return string_of_bits;
}

void comparison(unsigned char answers)
{
	std::println("You could be using 8 bytes of storage:");
	std::string i_readed;
	for(short i = 0; i < 8; i++)
	{
		if (read(answers, i) == 1)
		{
			i_readed = "1";
		}else if (read(answers, i) == 0)
		{
			i_readed = "0";
		}else{std::println("Error in this patch of sh***");}
		std::print("0000000");
		std::println("{}", i_readed);
	}
}

int main()
{
	std::println("The following questions are [y]es or [n]ot answers:");
	unsigned char answers = 0;
	for(short i = 0; i < 8 ; i++)
	{
		if (interviewer(i))
		{
			answers = write(answers, i, true);
		}else{
			answers = write(answers, i, false);
		}
	}
	std::print("\n");
	comparison(answers);
	std::println("\nInstead you're using only one: {}", list(answers));
	std::println("				      |-> {} in decimal.", answers);
}
