# dndlang

Dndlang is a language designed to create an import characters, play characters,
write scripts to automate character actions, and keep track of game state.

## Language Design

dndlang is a dynamically typed and garbage collected scripting language which
supports dnd-specific operator overloading (rolling, checks, attacks, etc.),
as well as more traditional operator overloading.

Since it is dynamically typed (with optional hints, like python), type checking
operators will be provided (typeof). types will be of `typeof type`. 

## Builtins
There are several builtin functions
`print`, `roll`, `range`, `foreach`, `map`, `roll_dc` # TODO
