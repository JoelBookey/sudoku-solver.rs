# sudoku-solver.rs
Brute force algorithm for solving sudoku puzzles.

Goes through the grid and at a None value, it iterates through the possible numbers and then at the next None value it does the same,
if there is no possible numbers then it returns.

There is a seperate mode called hard check that solves the grid but continues to see if there is another solution, this is because a valid sudoku puzzle only has one solution. 

## Input File
Use underscores for blank spaces, see test.txt for an example.

## Usage
     cargo run <filename> 
This will solve the puzzle and print out the answer.

     --hard
This will use the hard check.

     --debug
This will print the grid at each stage of the solver.

![Alt Text](images/sudoku-debug.gif)
