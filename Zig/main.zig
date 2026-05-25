const std = @import("std");
const eql = @import("std.mem.eql");

pub fn main() void {
    const questions_db: [8][]const u8 = .{ "Is allocating in the heap faster than pushing to the stack", "A pointer is a reference in memory to an actual value", "Function calling could be maded with jmp instructions", "This program is runing from your hard storage", "Concurrency allow CPU's to manage multiple threads", "A thread is like another processor more", "A compiler translates a program into a series of CPU instructions each time it runs", "Is a Interpreted language slower than compiled ones" };

    const feedback: [8][]const u8 = .{ "Searching through, for freed space were the data fits is more complex and it bring more CPU instructions than storing a value and a pointer to were the data starts.", "A pointer is a variable that store the direction in memory of the data it refers.", "At low level, function calling and returning, is maded from jmp instructions, same as loops and conditionals", "This program run from intructions loaded from hard disk into DRAM, all the program is loaded while it runs, when finishes is cleaned", "Concurrency allow a CPU to have many proceses running concurrently so it gives the ilusion that are runnig at the same time", "A thread is a CPU core that could run in paralel with many others", "A compiler does translate statements into CPU instructions, the resultant is a binary file that could be runned multiple times", "Interpreted languajes are shit" };

    var answers: u8 = 0;
    const reference: u8 = 0b01100101;
    std.debug.print("The following questions are [y]es or [n]ot answered:\n", .{});

    std.debug.print("How do you want the feedback for your responces:\n [A] => At the end of the program.\n [B] => After each responce.\n", .{});

    var line_buffer: [1024]u8 = undefined;
    var where_feedback: std.io.Writer = .fixed(&line_buffer);

    // Read an input until "\n" or end of file, and write it to the buffer
    const line_length = try std.stdin.interface.streamDelimiterLimit(&where_feedback, "\n", .unlimited);
    const input_line = line_buffer[0..line_length];

    while (eql(u8, where_feedback, "A") and eql(u8, where_feedback, "B")) {
        std.debug.print("Not a valid responce, put C if you wanna get THE FUCK OUT OF HERE!, or A/B:\n", .{});

        line_length = try std.stdin.interface.streamDelimiterLimit(&where_feedback, "\n", .unlimited);
        input_line = line_buffer[0..line_length];

        if (eql(u8, where_feedback, "C")) {
            @panic("You decided to go fuck out!\n");
        } else {}
    }

    if (eql(u8, where_feedback, "A")) {
        var i: u8 = 0;
        while (i < 8) : (i += 1) {
            answers = write(answers, i, interviewer(i, questions_db[i]));
        }
        std.debug.print("\nCorrections:\n\n");
        var o: u8 = 0;
        while (o < 8) : (o += 1) {
            std.debug.print("0{}. {}?\n", .{o + 1}, .{questions_db[o]});
            correction(reference, answers, feedback[o], o);
        }
    } else if (eql(u8, where_feedback, "B")) {
        var i: u8 = 0;
        while (i < 8) : (i += 1) {
            const responce: bool = interviewer(i, questions_db[i]);
            answers = write(answers, i, responce);
            correction(reference, answers, feedback[i], i);
        }
    } else {
        unreachable;
    }

    comparison(answers, reference);
    std.debug.print("Instead your are using only one: [{}]\n", .{list(answers)});
}

fn list(answers_registry: u8) [8]u8 {
    var i: u8 = 7;
    const string_registry: [8]u8 = undefined;
    while (i >= 0) : (i -= 1) {
        switch (read(answers_registry, i)) {
            1 => string_registry ++ "1",
            0 => string_registry ++ "0",
            _ => unreachable,
        }
    }
    return string_registry;
}

fn comparison(answers_registry: u8, reference: u8) noreturn {
    std.debug.print("You could be using 8 bytes of memory:\n", .{});
    var i: u8 = 0;
    while (i < 8) : (i += 1) {
        std.debug.print("[0000000", .{});

        switch (read(answers_registry, i)) {
            1 => std.debug.print("1] --> (true)", .{}),
            0 => std.debug.print("0] --> (false)", .{}),
            _ => unreachable,
        }

        if (read(answers_registry, i) != read(reference, i)) {
            std.debug.print(" => (Wrong)\n", .{});
        } else {
            std.debug.print(" => (Correct)\n", .{});
        }
    }
}

fn correction(reference: u8, answers_registry: u8, feedback: []u8, index: u8) void {
    if (read(answers_registry, index) != read(reference, index)) {
        std.debug.print("=> Wrong!\n", .{});
    } else {
        std.debug.print("=> Correct!\n", .{});
    }
    std.debug.print("¿*? {s}\n", .{feedback});
}

fn interviewer(index: u8, question: []u8) bool {
    std.debug.print("0{a}. {b}? > ", .{index + 1}, .{question});

    var line_buffer: [1024]u8 = undefined;
    var answer: std.io.Writer = .fixed(&line_buffer);

    // Read an input until "\n" or end of file, and write it to the buffer
    try std.stdin.interface.streamDelimiterLimit(&answer, "\n", .unlimited);

    if (eql(u8, answer, "y")) {
        return true;
    } else if (eql(u8, answer, "n")) {
        return false;
    } else {
        std.debug.print("That didn't seen like a valid responce, again!\n", .{});
        return interviewer(index);
    }
}

fn write(answers_registry: u8, possition: u8, digit: bool) u8 {
    switch (digit) {
        true => return (answers_registry | (1 << possition)),
        false => return (answers_registry & ~(1 << possition)),
    }
}

fn read(answers_registry: u8, possition: u8) u8 {
    return ((answers_registry >> possition) & 1);
}
