/*
/r/DailyProgrammer Challenge: 
(found at https://www.reddit.com/r/dailyprogrammer/comments/6v29zk/170821_challenge_328_easy_latin_squares/)

A Latin square is an n × n array filled with n different symbols, each occurring 
exactly once in each row and exactly once in each column.
For example:
1
And,
1 2
2 1
Another one, 
1 2 3 
3 1 2
2 3 1
In this challenge, you have to check whether a given array is a Latin square. 

Input Description:
	Let the user enter the length of the array followed by n x n numbers. 
	Fill an array from left to right starting from above. 

Output Description:
	If it is a Latin square, then display true. Else, display false. 
*/
package main

import "fmt"

func main() {
	// Read in 'n'
	fmt.Print("Enter number or rows/columns: ")
	var n int
	_, error := fmt.Scanln(&n)
	if error != nil {
		fmt.Println(error.Error())
		return
	}
	fmt.Println(n)

	// Read in the square
	square := make([]int, n*n)
	for x := 0; x < n; x++ {
		for y := 0; y < n; y++ {
			_, error := fmt.Scan(&square[x*n+y])
			if error != nil {
				fmt.Println(error.Error())
				return
			}
		}
	}

	printSquare(square, n)
	// Run the algorithm
	fmt.Println(isLatinSquare(square, n))
}

// printSquare prints the square out in a human-readable way, to allow the
// user to see what square they gave as input.
func printSquare(square []int, n int) {
	for x := 0; x < n; x++ {
		for y := 0; y < n; y++ {
			element := square[x*n+y]
			fmt.Print(element)
			fmt.Print(" ")
		}
		fmt.Println()
	}
}

// isLatinSquare takes an array and its width/height, and checks to see if
// it represents a Latin Square.
func isLatinSquare(array []int, n int) bool {
	// Check to ensure it contains n different symbols:
	{
		// Build up a map of values to number of times it occurs
		symbols := make(map[int]int)
		for i := 0; i < len(array); i++ {
			element := array[i]
			count, ok := symbols[element]
			if ok {
				symbols[element] = count + 1
			} else {
				symbols[element] = 1
			}
		}
		// Check to see each value in the map == n
		for _, count := range symbols {
			if count != n {
				return false
			}
		}
	}

	// Check each column:
	for y := 0; y < n; y++ {
		var column []int
		for x := 0; x < n; x++ {
			value := array[x*n+y]
			if contains(column, value) {
				return false
			}
			column = append(column, value)
		}
	}

	// Check each row:
	for x := 0; x < n; x++ {
		var row []int
		for y := 0; y < n; y++ {
			value := array[x*n+y]
			if contains(row, value) {
				return false
			}
			row = append(row, value)
		}
	}
	// If we have gotten this far, it's a latin square.
	return true
}

// contains checks to see if an int slice contains a specific value.
func contains(list []int, value int) bool {
	for i := 0; i < len(list); i++ {
		if list[i] == value {
			return true
		}
	}
	return false
}
