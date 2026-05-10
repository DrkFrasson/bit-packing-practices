#include <iostream>
#include <string>
#include <print>

bool interviewer(short index)
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
	for(short i = 0; i < 8; i++)
	{
//		std::print("0000000");
		std::cout << "0000000" << read(answers, i) << std::endl;
//		std::println("{}", read(answers, i));
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
	std::println("Your responce was: {}", list(answers));
	std::println("Or {} in decimal.", answers);
	comparison(answers);
}
