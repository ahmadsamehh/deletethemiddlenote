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
Clone the repo from github and replace the inpuit vector with your own, then let the magic happen 😂.

you can use this to clone the project :-

gh repo clone ahmadsamehh/deletethemiddlenote

