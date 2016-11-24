# infix_to_postfix
Converts an infix expression into a postfix expression.
Uses this algorithm:
1. Create a stack.
2. Scan the tokens in the infix expression.
  * If the token is an operand, output it.
  * If the token is a left parenthesis, push it onto the stack.
  * If the token is a right parenthesis, pop and output all the operators from the stack until encountering a left parenthesis. Pop and discard the left parenthesis.
  * If the token is an operator
	* If the stack is empty, push the token onto the stack.
	* Otherwise, if the top of the stack is an operator and its precedence is greater than or equal to the precedence of the token, pop and output the operator from the stack, and repeat this process. Finally, push the token in the infix expression onto the stack.
3. Pop and output all the remaining tokens on the stack.
