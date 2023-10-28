package lexer

import (
	"testing"

	"github.com/ibado/monkey/token"
)

func TestNextToken(t *testing.T) {
	input := `let five = 5;
	let ten = 10;

	let add = fn(x, y) {
		return x + y;
	};

	let result = add(five, ten);
	!-/*5;
	5 < 10 > 5;
	let r;
	if (result < 16) {
		r = true	
	} else {
		r = false	
	}
	== !=
	`

	tests := []struct {
		expectedType    token.TokenType
		expectedLiteral string
	}{
		{token.LET, "let"},
		{token.IDENTIFIER, "five"},
		{token.ASSIGN, "="},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENTIFIER, "ten"},
		{token.ASSIGN, "="},
		{token.INT, "10"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENTIFIER, "add"},
		{token.ASSIGN, "="},
		{token.FUNCTION, "fn"},
		{token.OPEN_PARENTHESIS, "("},
		{token.IDENTIFIER, "x"},
		{token.COMMA, ","},
		{token.IDENTIFIER, "y"},
		{token.CLOSE_PARENTHESIS, ")"},
		{token.OPEN_BRACE, "{"},
		{token.RETURN, "return"},
		{token.IDENTIFIER, "x"},
		{token.PLUS, "+"},
		{token.IDENTIFIER, "y"},
		{token.SEMICOLON, ";"},
		{token.CLOSE_BRACE, "}"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENTIFIER, "result"},
		{token.ASSIGN, "="},
		{token.IDENTIFIER, "add"},
		{token.OPEN_PARENTHESIS, "("},
		{token.IDENTIFIER, "five"},
		{token.COMMA, ","},
		{token.IDENTIFIER, "ten"},
		{token.CLOSE_PARENTHESIS, ")"},
		{token.SEMICOLON, ";"},
		{token.BANG, "!"},
		{token.MINUS, "-"},
		{token.SLASH, "/"},
		{token.ASTERISK, "*"},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.INT, "5"},
		{token.LT, "<"},
		{token.INT, "10"},
		{token.GT, ">"},
		{token.INT, "5"},
		{token.SEMICOLON, ";"},
		{token.LET, "let"},
		{token.IDENTIFIER, "r"},
		{token.SEMICOLON, ";"},
		{token.IF, "if"},
		{token.OPEN_PARENTHESIS, "("},
		{token.IDENTIFIER, "result"},
		{token.LT, "<"},
		{token.INT, "16"},
		{token.CLOSE_PARENTHESIS, ")"},
		{token.OPEN_BRACE, "{"},
		{token.IDENTIFIER, "r"},
		{token.ASSIGN, "="},
		{token.TRUE, "true"},
		{token.CLOSE_BRACE, "}"},
		{token.ELSE, "else"},
		{token.OPEN_BRACE, "{"},
		{token.IDENTIFIER, "r"},
		{token.ASSIGN, "="},
		{token.FALSE, "false"},
		{token.CLOSE_BRACE, "}"},
		{token.EQ, "=="},
		{token.NEQ, "!="},
		{token.EOF, ""},
	}

	lexer := New(input)
	for i, tt := range tests {
		token := lexer.NextToken()

		if token.Type != tt.expectedType {
			t.Fatalf(
				"tests[%d] - wrong literal. Expected= %q, actual=%q",
				i, tt.expectedType, token.Type,
			)
		}
	}
}
