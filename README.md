### rolyng_basic_make_parser

Parser of basic make syntax for educational purpose

### Description

* Make syntax parser created to parse basic Makefiles
* Supports basic targets, dependencies and commands

### Grammar

```pest
file = {SOI ~ line+ ~ EOI}
line = {(EMPTY_LINE | rule)}
rule = {rule_header ~ recipe}
rule_header = {rule_name ~ ":" ~ dependencies? ~ "\n"}
dependencies = {identifier ~ ("," ~ identifier)*}
rule_name = {identifier}
recipe = {EMPTY_LINE | (recipe_command+)}
recipe_command = { RECIPE_PREFIX ~ shell_command ~ "\n"}
shell_command = @{(!"\n" ~ ANY)+}
identifier = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }

RECIPE_PREFIX = _{"\t"}
EMPTY_LINE = _{"\n"}
WHITESPACE = _{ " " }
COMMENT = _{ "#" ~ (!"\n" ~ ANY)*}
```

* file - main rule to parse Makefile
* line - rule for each text line. Can be empty or a make rule
* rule - rule consisting of header and recipe
* rule_header - header of a make rule which has it's name and dependencies
* dependencies - in this basic parser it's basically identifiers separated by commands
* rule_name - identifier for the name of rule
* recipe - can be either empty or consists of some recipe_commands
* recipe_command - starts with RECIPE_PREFIX which is \t in usual make and after contains command to be executed in shell. Ends with newline
* shell_command - anything not containing newline 
* identifier - starts with symbol and continues with any alphanumeric or '_'. also cant contain WHITESPACE

### Usage 

```sh
./rolyng_basic_make_parser -h or --help or help #show help
./rolyng_basic_make_parser -V #show version
./rolyng_basic_make_parser -f or --file <file> #parse a file
./rolyng_basic_make_parser author #prints author
```

### Example

```make
#comment
aaa: bbb, ccc #comment
	echo ab

bbb: adsa
	echo bbb

ccc:
	echo ccc
	cat boba

ddd:
```

```
Rule 
	name: aaa
	dependencies: bbb,ccc,
	commands:
		echo ab
		
Rule 
	name: bbb
	dependencies: adsa,
	commands:
		echo bbb
		
Rule 
	name: ccc
	dependencies: 
	commands:
		echo ccc
		cat boba
		
Rule 
	name: ddd
	dependencies: 
	commands:
		
```
