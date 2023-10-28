package main

import (
	"bufio"
	"fmt"
	"os"

	"github.com/ibado/monkey/lexer"
	"github.com/ibado/monkey/token"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	for {
		fmt.Print(">> ")

		if !scanner.Scan() {
			return
		}

		lexer := lexer.New(scanner.Text())
		for {
			tok := lexer.NextToken()
			if tok.Type == token.EOF {
				break
			}
			fmt.Println(tok)
		}
	}
}
