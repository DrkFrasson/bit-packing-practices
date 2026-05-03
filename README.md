# bit-packing practices
bit-packing is a method to store more data in one variable to leverage all it's capacity,
for example you could have a bunch of `bool` variables.

bool data type: byte
[00000000] -> false
[00000001] -> true

And there we are wasting 7 bits of valuable space.

This could help in embeded systems that have a limited storage capacity.

In this program this didn't have a real advantage, rater than adding complexity, but this is only a practice what i'm implementing to a future project that maybe help.

# Sources:
I learned all this from this awesome video from [Core Dumped](https://www.youtube.com/@CoreDumpped):
[Why Some low-level Projects Are Full of Weird Code Like This](https://www.youtube.com/watch?v=z7wVUfnm7M0&pp=0gcJCd4KAYcqIYzv)
