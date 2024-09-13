package main

import "fmt"

func main() {
	var str string
	fmt.Println("Hello world!")
	fmt.Print(">")
	fmt.Scan(&str)
	fmt.Printf("you enter: %s\n", str)
}
