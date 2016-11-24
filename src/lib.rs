#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
}

#[derive(Debug, PartialEq)]
pub enum InfixToken {
    Operator(Operator),
    Operand(isize),
    LeftParen,
    RightParen,
}

#[derive(Debug, PartialEq)]
pub enum PostfixToken {
    Operator(Operator),
    Operand(isize),
}

/// Transforms an infix expression to a postfix expression.
///
/// If the infix expression is valid, outputs `Some(_)`; 
/// otherwise, outputs `None`.
pub fn infix_to_postfix(tokens: &[InfixToken]) -> Option<Vec<PostfixToken>> {
    let mut stack: Vec<InfixToken> = vec![];
    let mut output: Vec<PostfixToken> = vec![];
    



//////////////////////////////////   VALIDATION   ///////////////////////////////////////////////////
    let mut right_paren = 0;
    let mut left_paren = 0;
    for token in tokens{
    	if token == &InfixToken::RightParen{
    		right_paren+=1;
    	}
    	if token == &InfixToken::LeftParen{
    		left_paren+=1;
    	}
    }
    if right_paren != left_paren {
    	return None;
    }

    if tokens[0] == InfixToken::RightParen{
    	return None;
    }
    if tokens[0] == InfixToken::Operator(Operator::Add){
    	return None;
    }
    if tokens[0] == InfixToken::Operator(Operator::Sub){
    	return None;
    }
    if tokens[0] == InfixToken::Operator(Operator::Mul){
    	return None;
    }
    if tokens[0] == InfixToken::Operator(Operator::Div){
    	return None;
    }




    if tokens[tokens.len()-1] == InfixToken::LeftParen{
    	return None;
    }
    if tokens[tokens.len()-1] == InfixToken::Operator(Operator::Add){
    	return None;
    }
    if tokens[tokens.len()-1] == InfixToken::Operator(Operator::Sub){
    	return None;
    }
    if tokens[tokens.len()-1] == InfixToken::Operator(Operator::Mul){
    	return None;
    }
    if tokens[tokens.len()-1] == InfixToken::Operator(Operator::Div){
    	return None;
    }

   let mut index = 0;
   while index != tokens.len()-1{
   		if tokens[index] == InfixToken::RightParen && tokens[index+1] == InfixToken::LeftParen{
   			return None;
   		}	
   		index += 1;
   }

   let mut index = 0;
   while index != tokens.len()-1{
   		let ref k = tokens[index];
   		match k{
   			&InfixToken::Operand(x) => {
   				if tokens[index+1] == InfixToken::Operand(x){
   					return None;
   				}
   				if tokens[index+1] == InfixToken::LeftParen{
   					return None;
   				}
   				
   			}
   			&InfixToken::Operator(operator) => {
   				if tokens[index+1] == InfixToken::Operator(operator){
   					return None;
   				}
   				if tokens[index+1] == InfixToken::RightParen{
   					return None;
   				}
   			}
   			&InfixToken::LeftParen => {
   				if tokens[index+1] == InfixToken::RightParen{
   					return None;
   				}
   			}
   			_ => {},


   		}
   		index += 1;
   }


   let mut index = 1;
   while index != tokens.len(){
   		let ref k = tokens[index];
   		match k{
   			&InfixToken::Operand(x) => {
   				
   				if tokens[index-1] == InfixToken::RightParen{
   					return None;
   				}
   			}
   			&InfixToken::Operator(x) => {
   				if tokens[index-1] == InfixToken::LeftParen{
   					return None;
   				}
   			}
   			_ => {},
   		}
   		index += 1;
   }





    

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

    for token in tokens{
    	match token{
    		&InfixToken::Operand(x) => output.push(PostfixToken::Operand(x)),
    		&InfixToken::LeftParen => stack.push(InfixToken::LeftParen),
   		
    		&InfixToken::RightParen => 
    		{
    			loop{

    					let y = stack.pop().unwrap();
    					match y{
    						InfixToken::Operator(y) => output.push(PostfixToken::Operator(y)),
    						InfixToken::LeftParen => break,
    						InfixToken::RightParen => break,
    						InfixToken::Operand(x) => break,
    					}
    				}
    			
    		},
    		
    		&InfixToken::Operator(Operator::Add) => 
    		{
    			if stack.len() == 0{
    				stack.push(InfixToken::Operator(Operator::Add));		
    			}
    			////////////////////////////////////////////////
    			else{

    				loop{
    					
    					if stack.len() == 0{
    						break;
    					}

    					let z = stack.pop().unwrap();
    					
    					match z{
    						InfixToken::Operator(Operator::Add) => {
    							output.push(PostfixToken::Operator(Operator::Add));
    							//stack.push(InfixToken::Operator(Operator::Add));
    						},
    						InfixToken::Operator(Operator::Sub) => {
    							output.push(PostfixToken::Operator(Operator::Sub));
    							//stack.push(InfixToken::Operator(Operator::Add));
    						},

    						InfixToken::Operator(Operator::Mul) => {
								output.push(PostfixToken::Operator(Operator::Mul));
								//stack.push(InfixToken::Operator(Operator::Add));
							}
    						InfixToken::Operator(Operator::Div) => {
    							output.push(PostfixToken::Operator(Operator::Div));
    							//stack.push(InfixToken::Operator(Operator::Add));
    						},

    						InfixToken::LeftParen => {
    							stack.push(InfixToken::LeftParen);
    							//stack.push(InfixToken::Operator(Operator::Add));
    							break;
    						}
    						InfixToken::RightParen => {
    							stack.push(InfixToken::RightParen);
    							//stack.push(InfixToken::Operator(Operator::Add));
    							break;
    						}
    						_ => {},
    						
    					}
    				}
    				stack.push(InfixToken::Operator(Operator::Add));

    				
    			
    			}
/////////////////////////////////////////////////////////////////
    		},

    		&InfixToken::Operator(Operator::Sub) => 
    		{
    			if stack.len() == 0{
    				stack.push(InfixToken::Operator(Operator::Sub));		
    			}
    			////////////////////////////////////////////////
    			else{

    				loop{

    					if stack.len() == 0{
    						break;
    					}

    					let z = stack.pop().unwrap();
    					
    					match z{
    						InfixToken::Operator(Operator::Add) => {
    							output.push(PostfixToken::Operator(Operator::Add));
    							//stack.push(InfixToken::Operator(Operator::Sub));
    						},
    						InfixToken::Operator(Operator::Sub) => {
    							output.push(PostfixToken::Operator(Operator::Sub));
    							//stack.push(InfixToken::Operator(Operator::Sub));
    						},

    						InfixToken::Operator(Operator::Mul) => {
								output.push(PostfixToken::Operator(Operator::Mul));
								//stack.push(InfixToken::Operator(Operator::Sub));
							}
    						InfixToken::Operator(Operator::Div) => {
    							output.push(PostfixToken::Operator(Operator::Div));
    							//stack.push(InfixToken::Operator(Operator::Sub));
    						},
    						InfixToken::LeftParen => {
    							stack.push(InfixToken::LeftParen);
    							//stack.push(InfixToken::Operator(Operator::Sub));
    							break;
    						}
    						InfixToken::RightParen => {
    							stack.push(InfixToken::RightParen);
    							//stack.push(InfixToken::Operator(Operator::Sub));
    							break;
    						}
    						_ => {},
    						
    					}
    				}
    				stack.push(InfixToken::Operator(Operator::Sub));
    				
    			
    			}
/////////////////////////////////////////////////////////////////
    		},

    		&InfixToken::Operator(Operator::Mul) => 
    		{
    			if stack.len() == 0{
    				stack.push(InfixToken::Operator(Operator::Mul));		
    			}
    			////////////////////////////////////////////////
    			else{

    				loop{

    					if stack.len() == 0{
    						break;
    					}

    					let z = stack.pop().unwrap();
    					
    					match z{
    						InfixToken::Operator(Operator::Add) => {
    							stack.push(InfixToken::Operator(Operator::Add));
    							//stack.push(InfixToken::Operator(Operator::Mul));
    							break;
    						}
    						InfixToken::Operator(Operator::Sub) => {
    							stack.push(InfixToken::Operator(Operator::Sub));
    							//stack.push(InfixToken::Operator(Operator::Mul));
    							break;
    						}
    						InfixToken::Operator(Operator::Mul) => {
    							output.push(PostfixToken::Operator(Operator::Mul));
    							//stack.push(InfixToken::Operator(Operator::Mul));
    						}

    						InfixToken::Operator(Operator::Div) => {
    							output.push(PostfixToken::Operator(Operator::Div));
    							//stack.push(InfixToken::Operator(Operator::Mul));
    						}
    						InfixToken::LeftParen => {
    							stack.push(InfixToken::LeftParen);
    							//stack.push(InfixToken::Operator(Operator::Mul));
    							break;
    						}
    						InfixToken::RightParen => {
    							stack.push(InfixToken::RightParen);
    							//stack.push(InfixToken::Operator(Operator::Mul));
    							break;
    						}
    						_ => {},
    						
    					}
    				}
    				stack.push(InfixToken::Operator(Operator::Mul));
    				
    			
    			}
/////////////////////////////////////////////////////////////////
    		},

    		&InfixToken::Operator(Operator::Div) => 
    		{
    			if stack.len() == 0{
    				stack.push(InfixToken::Operator(Operator::Div));		
    			}
    			////////////////////////////////////////////////
    			else{

    				loop{

    					if stack.len() == 0{
    						break;
    					}

    					let z = stack.pop().unwrap();
    					
    					match z{
    						InfixToken::Operator(Operator::Add) => {
    							stack.push(InfixToken::Operator(Operator::Add));
    							//stack.push(InfixToken::Operator(Operator::Div));
    							break;
    						}
    						InfixToken::Operator(Operator::Sub) => {
    							stack.push(InfixToken::Operator(Operator::Sub));
    							//stack.push(InfixToken::Operator(Operator::Div));
    							break;
    						}
    						InfixToken::Operator(Operator::Mul) => {
    							output.push(PostfixToken::Operator(Operator::Mul))
    						
    						},
    						InfixToken::Operator(Operator::Div) => {
    							output.push(PostfixToken::Operator(Operator::Div))
    						},
    						InfixToken::LeftParen => {
    							stack.push(InfixToken::LeftParen);
    							//stack.push(InfixToken::Operator(Operator::Div));
    							break;
    						}
    						InfixToken::RightParen => {
    							stack.push(InfixToken::RightParen);
    							//stack.push(InfixToken::Operator(Operator::Div));
    							break;
    						}
    						_ => {},
    						
    					}
    				}
    				stack.push(InfixToken::Operator(Operator::Div));
    				
    			
    			}
/////////////////////////////////////////////////////////////////
    		},


    		

    	}

    }
    while stack.len() != 0{
    	let x = stack.pop().unwrap();
    	match x{
    		InfixToken::Operator(x) => output.push(PostfixToken::Operator(x)),
    		_ => {},//return None,////////////////////////////////////////////////////////////////////////////////
    	}
    }

    

    return Some(output);
}

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
    	let v: Vec<PostfixToken> = vec![PostfixToken::Operand(2),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add)];
    	assert_eq!(Some(v), infix_to_postfix(&[InfixToken::Operand(2),InfixToken::Operator(Operator::Add),InfixToken::Operand(3)]))
    }
    #[test]
    fn it_works1() {
    	let v: Vec<PostfixToken> = vec![PostfixToken::Operand(2),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(1),PostfixToken::Operator(Operator::Sub)];
    	assert_eq!(Some(v), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(2),InfixToken::Operator(Operator::Add),InfixToken::Operand(3),InfixToken::RightParen,InfixToken::Operator(Operator::Sub),InfixToken::Operand(1)]))
    }
    #[test]
    fn it_works2() {
    	let v: Vec<PostfixToken> = vec![PostfixToken::Operand(2),PostfixToken::Operand(3),PostfixToken::Operator(Operator::Add),PostfixToken::Operand(2),PostfixToken::Operand(1),PostfixToken::Operator(Operator::Add),PostfixToken::Operator(Operator::Sub)];
    	assert_eq!(Some(v), infix_to_postfix(&[InfixToken::LeftParen,InfixToken::Operand(2),InfixToken::Operator(Operator::Add),InfixToken::Operand(3),InfixToken::RightParen,InfixToken::Operator(Operator::Sub),InfixToken::LeftParen,InfixToken::Operand(2),InfixToken::Operator(Operator::Add),InfixToken::Operand(1),InfixToken::RightParen]))
    }
    
}



