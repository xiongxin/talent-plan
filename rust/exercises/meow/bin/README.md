### App describe the top level application

## Args

Args describe a possible valid argument which my be supplied by the user at runtime.
There are three different types of arguments
- flags
- options
- positional
as well as fourth special type of argument, called SubCommand.



## Using Matches

Once all App settings (including all arguments) have been set, you call get_matches()
which parses the string provided by the user, and returns all the valid matches to
the ones you specified.

You can then query the matches struct to get information about how the user ran the
program at startup.

## Flag Args

Of the three argument types, flags are the most simple. Flags are simple switches
which can be either "on" or "off"

## Positional Args

Positional arguments are those values after the program name which are not preceded
by any identifier (such as "myapp some_file"). Positional support many of the same
option as flags, as well as a few additional ones.

## Option Args

Option arguments are those that take an additional value, such as "-c value".
In clap they support thre types of specification
- short() as "-o some"
- long() as "--option value" or "--option=value"

Options also support a multiple setting, which is discussed in the example below.


