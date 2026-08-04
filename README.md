# regvm
An virtual machine modeling a very simple registry machine.
I wrote this on a single day without any AI help or researching anything about assembly languages.

When running the binary, it reads in a text file called "program" in the same directory and runs it.

There is currently only one type of value, that is an unsigned integer.

# Instructions

``wrt <reg> <value>`` Stores `value` in the given register `reg`

``cpy <reg_src> <reg_target>`` Stores the value of `reg_src` in `reg_target`  

``add <a> <b> <reg>`` Computes the sum of a + b and stores it in the given register `reg`

``sub <a> <b> <reg>`` Computes the difference of a - b and stores it in the given register `reg`

``print <addr>`` Prints the value at the register

Plain text is considered a `marker` instruction with the text as its name

``go <reg> <marker1> <marker2>`` If the value at register `reg` is 0, jump to marker1, else jump to marker2