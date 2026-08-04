# regvm
An virtual machine modeling a very simple registry machine.
I wrote this on a single day without any AI help or researching anything about assembly languages.

There is currently only one type of value, that is an unsigned integer.

# Instructions

``wrt <reg> <value>`` Stores `value` in the given register `reg`

``cpy <reg_src> <reg_target>`` Stores the value of `reg_src` in `reg_target`  

``add <a> <b> <reg>`` Computes the sum of a + b and stores it in the given register `reg`

``sub <a> <b> <reg>`` Computes the difference of a - b and stores it in the given register `reg`

``print <addr>`` Prints the value at the register

Plain text is considered a `marker` instruction with the text as its name

``go <reg> <marker1> <marker2>`` If the value at register `reg` is 0, jump to marker1, else jump to marker2

# Run

Run a program by passing in the program file like this:

If youre in the directory of the cloned repo: 

``cargo run --release -- <program_file>``

Or if you have the binary in path:

``regvm <program_file>``
