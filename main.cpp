#include <iostream>
#include <string>
#include <print>

bool interviewer(unsigned char index)
{
	std::string answer;

	std::string questions_db[8] = {
	"Does exist micro-controlers with 64 bits architecture",
	"Is Processing like a dialect of C++",
	"idk3",
	"idk4",
	"idk5",
	"idk6",
	"idk7",
	"idk8",
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

char read(unsigned char word, unsigned char possition){return (word >> possition) & 1;}

// <--<--<--<--

std::string list (unsigned char word)
{
	std::string string_of_bits;

	for(short i = 0; i < 8; i++)
	{
		/*
		switch (read(word, i)):
			case 0: string_of_bits + "0",
			case 1: string_of_bits + "1",
		*/

		string_of_bits += read(word, i);
	}

	return (string_of_bits);
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


	std::println("Your responce was: {}", list(answers));
	std::println("Or {} in decimal.", answers);
}
