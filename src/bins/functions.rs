/*  
Differences between Expressions and Statements

Expressions
 -Evaluates to a Value eg 5+6
 -No Semi-colon  x*y

Statements
 -Perform an Action eg Let x= 5;
 -Semi-colon


Note:
Expressions can be part of statments but statements can't be part  of expressions

Possible compile errors
let y = (let x = 3);  // This will not compile because `let x = 3` is a statement.


*/




fn main() {
    //println!("Hello, world!");

    another_function(-5);
	print_labeled_measurement(5,'m');
	expression_value();
	let return_value = return_variable();
	println!("The value of returnValue  is: {return_value}");
	
	
}

fn another_function(x:i32) {
    //println!("Another function.");
	println!("The value of x is: {x}")
}

fn print_labeled_measurement(value:i32, unit_label:char){
	println!("The measurment is: {value}{unit_label}");
}

fn expression_value(){
	let y= {
		let x=3;
		x+ 1
	};
	println!("The value of y is: {y}");
}


fn return_variable()->i32{
	5
}
