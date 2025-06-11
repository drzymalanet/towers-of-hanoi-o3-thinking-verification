# towers-of-hanoi-o3-thinking-verification

Verifying the statement of Ves Roth in a youtube video titled:
**o3 pro is a BEAST... one-shots Apple's "Illusion of Thinking" test"**

[link to youtube video](https://www.youtube.com/watch?v=vmrm90u0dHs)

Author Wes Roth states that the shared prompt for the 10 disk Tower of Hanoi: https://chatgpt.com/share/6848fff7-0080-8013-a032-e18c999dc371 is proving wrong the scientific paper titled "The illusion of thinking".

After checking the result data from the link provided one can see that indeed the paper from Apple is correct
and the o3 model is not capable of solving the puzzle.

The invalid move is the move number 96:
```
Move number 88: Moving disk 4 from peg 1 to peg 0
Move number 89: Moving disk 1 from peg 2 to peg 0
Move number 90: Moving disk 2 from peg 2 to peg 1
Move number 91: Moving disk 1 from peg 0 to peg 1
Move number 92: Moving disk 3 from peg 2 to peg 0
Move number 93: Moving disk 1 from peg 1 to peg 2
Move number 94: Moving disk 2 from peg 1 to peg 0
Move number 95: Moving disk 1 from peg 2 to peg 0
Move number 96: Moving disk 6 from peg 1 to peg 2

thread 'main' panicked at src/main.rs:24:13:
Invalid move: Cannot take 6: Top disk is 7
```
