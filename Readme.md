# Delete Middle Node

Solution to the "Delete Middle Node" problem on LeetCode, implemented in Rust.

## Problem Description

Given a linked list, delete the middle node of the list.

## Approach
The problem can be solved by getting the length of the input vector or list, dividing it by two and getting the whole number.
That whole number will be pointing to the position of the node that needs to be deleted.

## Example

Input: 1 -> 2 -> 3 -> 4 -> 5.
Output: 1 -> 2 -> 4 -> 5.

The length of the input vector is 5, divide that length by 2, you'll get 2.
So "2" is the index of the element that needs to be deleted from the vector as we already know vectors indexing starts with 0 so index 2 will be the third element in the vector which is 3.

## Usage

1. You MUST have Rust installed on your OS.
>you can check Rust installation here ***[https://www.rust-lang.org/tools/install]***

2. Clone the repo from github using this command : 
`gh repo clone ahmadsamehh/deletethemiddlenote`.

3. Open the `main.rs` file using your text editor and then change the numbers in the `node` variable you'll find.
4. Save the code and use the "Cargo Run" command in your terminal or cmd and let the magic happens.
5. This should be the output in your terminal:
```
UnEdited Node : [1, 3, 4, 7, 1, 2, 6, 13, 14, 15, 20, 23, 27]
element to be deleted : 6
Edited Node : [1, 3, 4, 7, 1, 2, 13, 14, 15, 20, 23, 27]
```




