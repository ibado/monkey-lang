package token

import "fmt"

type TokenType string

type Token struct {
	Type    TokenType
	Literal string
}

func (t Token) String() string {
	return fmt.Sprintf("%s = %s", string(t.Type), t.Literal)
}

const (
	ILLEGAL = "ILLEGAL"
	EOF     = "EOF"

	// identifiers + literals
	IDENTIFIER = "IDENTIFIER"
	INT        = "INT"

	// Operators
	ASSIGN   = "="
	PLUS     = "+"
	MINUS    = "-"
	BANG     = "!"
	ASTERISK = "*"
	SLASH    = "/"
	LT       = "<"
	GT       = ">"
	EQ       = "=="
	NEQ      = "!="

	//Delimiters
	COMMA             = ","
	SEMICOLON         = ";"
	OPEN_PARENTHESIS  = "("
	CLOSE_PARENTHESIS = ")"
	OPEN_BRACE        = "{"
	CLOSE_BRACE       = "}"

	// keywords
	FUNCTION = "FUNCTION"
	LET      = "LET"
	RETURN   = "RETURN"
	IF       = "IF"
	ELSE     = "ELSE"
	TRUE     = "TRUE"
	FALSE    = "FALSE"
)

var keywords = map[string]TokenType{
	"fn":     FUNCTION,
	"let":    LET,
	"return": RETURN,
	"if":     IF,
	"else":   ELSE,
	"true":   TRUE,
	"false":  FALSE,
}

func ResolveType(identifier string) TokenType {
	if tok, ok := keywords[identifier]; ok {
		return tok
	}

	return IDENTIFIER
}
