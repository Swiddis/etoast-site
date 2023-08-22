package main

import (
	"log"
	"os"
)

func main() {
	content, err := os.ReadFile("content/content.txt")
	if err != nil {
		log.Fatalf("%s", err)
	}

	log.Printf("%s", string(content))
}
