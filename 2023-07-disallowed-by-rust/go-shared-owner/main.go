package main

import (
	"fmt"
	"time"
)

func main() {
	c := make(chan map[string]int)

	m := make(map[string]int)
	m["apples"] = 4
	m["bananas"] = 5

	go func() {
		recv := <-c
		fmt.Println("how many apples? ", recv["apples"])
		fmt.Println("here is a cherry!")
		recv["cherries"] = 1
	}()

	c <- m

	// Main goroutine continues to access the map
	m["cherries"] = 3
	fmt.Println("Updated map in main:", m)

	time.Sleep(1 * time.Second)
}

