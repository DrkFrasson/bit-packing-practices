# bit-packing practices
bit-packing is a method to store more data in one variable to leverage all it's capacity,
for example you could have a bunch of `bool` variables.

bool data type: `byte`\
[00000000] -> false\
[00000001] -> true

And there we are wasting 7 bits of valuable space.

bit-packing take advantage to that space using some bitwise operators that act on the bit level rather than the complete word to edit the un-used bits that no data type could handle or the scope of your program.

This could help in embeded systems that have a limited storage capacity.

In this program this didn't have a real advantage, rather, adding complexity, because isn't storing nothing in hard disk and the additional instructions to the actual bit-packing part grows the size of the program that's loaded into memory, but this is only a practice what i'm implementing to a future project that maybe help's.

# Sources:
I learned all this from this awesome video from [Core Dumped](https://www.youtube.com/@CoreDumpped):
[Why Some low-level Projects Are Full of Weird Code Like This](https://www.youtube.com/watch?v=z7wVUfnm7M0&pp=0gcJCd4KAYcqIYzv)

Example:

possition = 3;
word = 01011100;

word >> possition; = 

010[1]1100 3-> 0000010[1]
         & 0000000 1 = 00000001 = 1;


possition = 4;

01[0]11100 4-> 0000001[0]
         & 0000000 1 = 00000000 = 0;

