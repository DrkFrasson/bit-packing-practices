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

# Repository Index:
```bash
DrkFrasson/bit-packing-practices/ $ ls ./*

./c_lang -> Simbolic Link to the actual place of the C++ version of the program.
./kotlin_lang -> Simbolic Link to the actual place of the Kotlin version of the program.(Coming soon!)
./README.md -> README of the repo. (This file)

./Rust/
    main.rs -> Version of the program in the Rust programming language.
    main* -> Binary compiled from ./main.rs

./C++/
    main.cpp -> Version of the program in the C++ programming language.
    a.out* -> Binary compiled from ./main.cpp
```
# Sources:
I learned all this from this awesome video from [Core Dumped](https://www.youtube.com/@CoreDumpped):
[Why Some low-level Projects Are Full of Weird Code Like This](https://www.youtube.com/watch?v=z7wVUfnm7M0&pp=0gcJCd4KAYcqIYzv)


