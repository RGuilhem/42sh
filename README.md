# FT\_SH

## TODO

- [ ] logging of whats going on
- [ ] options on invocation
- [ ] reading of options files
- [ ] understand bash grammar

## Man page equivalent

__NAME__

ftsh - Forty two shell

__SYNOPSIS__

ftsh [options] [command\_string | file]
- [ ] options
- [ ] command
- [ ] file

__OPTIONS__

-c command read from first non option command string

-i interactive shell

-l login shell

-s commands read from stdin

-v print input line as they are read

-x print command and their args as they are executed

-- disable option processing, remaining arguments are treated as filenames and arguments. - is equivalent to --


__DEFINITIONS__

blank
Space or tab

word
Sequence of chars considered as a unit (token)

name
word only consisting of alnum and underscore (identifier)

metacharacter
When unquoted separates words, one of those:
| & ; ( ) < > space tab newline

control operator
token parforming a control function, one of those:
|| & && ; ;; ;& ;;& ( ) | |& newline
