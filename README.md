I recently bought the book "Beginning Rust Programming" by Ric Messier, and am working through it slowly but surely!

# Table of Contents
1. [Chapter 1](#chapter1)
2. [Example2](#example2)
3. [Third Example](#third-example)
4. [Fourth Example](#fourth-examplehttpwwwfourthexamplecom)

## Chapter 1 - Game of Life - The Basics <a name="chapter1"></a>
1970 John Horton Conway 

**Game of Life Rule**
- If a cell is currently alive but has fewer than 2 neighbours, it dies because of a lack of support 
- If a cell is alive and has two or three neighbours, it survives the next generation 
- If a cell is alive and has more than three neighbours, it dies from over population 
- If a cell is dead but has exactly three neighbours, it comes back to life 

### Building a New Rust Package 
#### Step 1 - Create Package
~~~
cargo new life
~~~

This creates a package that has:
1) A directory called src, which contains the main entry point file called main.rs. The complier looks for main() as the entry point 
2) A Cargo.toml file - metadata about the executable (package, dependencies)

*NB: Use lower case when naming a package, no Pascal or camel case*

#### Step 2 - Build Executable
~~~
cargo build
~~~

This builds the debug executable 
~~~
cargo build --release 
~~~

This builds the release executable

When we run cargo build, we will also get a target folder (target/debug/) which will have the name of the program with no extension. This is our executable, we can just run this by saying 
~~~
./target/debug/<name_of_program>
~~~
or when developing, 
~~~
rustc ./src/main.rs
~~~

### External Functionality 
i.e. code we didn't/ don't have to write ourselves. 

~~~
extern crate <extern_package_name>;
use std::<lib_package_name>;
use std::{<lib_package_name_1>, <lib_package_name_2>};
~~~

Rust libraries are called crates. The word extern means that the compiler has to look elsewhere for the code, and Cargo is the package manager. 

If we add extern crate, we have to reference it in our Cargo.toml file, including the name and the version 
~~~ Cargo.toml
[dependencies]
name_of_package = "0.0.0"
~~~

#### Namespaces 
Namespace is eseentially a container. Namespaces make sure that when our program gets larger, we don't end up creating functions and variables that clash. If they are separated by containers, we reduce the risk of using names for things twice. 
*NB: Everything is explicit in Rust!!*
*NB: The main function can go anywhere in the main.rs file, but by convention it goes at the bottom*

### Back to the Game 
Okay back to the elements of the game. 

#### Variables 
Good practice to define all variables at the top of a function, so basically right after the open curly bracket. Okay so here are some of the key points. 

~~~
let mut world = [[0u8; 75]; 75];
let mut generations = 0;
~~~

1) Declare a variable using the keyword ```let```
2) We can set it to something and let Rust infer the datatype, (like generations above) 
3) Use the word ```mut``` to tell Rust that the variable will be changing 


*NB: Difference between a constaht and an immutable variable is that a constant acts like an alias and the compiler just replaces the value at runtime, whereas an immutable variable is a variable that cannot be changed durring runtime*

#### Datatypes 
The first datatype we have encountered is the 0u8. 

~~~
let mut world = [[0u8; 75]; 75];
~~~

This means that the datatype will be initialised to 0, it will be unsigned (hence the ```u``` and it will be an 8 bit integer). This means we won;t expect it to be a value larger than 255. 11111111 is equal to 128-64-32-16-8-4-2-1, which is 255. 

Other integers we can use are: 
- i8 (i is signed meaning it can be positive or negative)
- i16
- i32
- i64
- i128
- u8
- u16
- u32
- u64
- u128

For floating point variables, we can use: 
- f32
- f64 (default as no major performance issues but a lot more precision)

A Char in Rust is a 4 byte value. 

Boolean definition are as follows: 

~~~
let yes_no: bool = true;
let yes_no = true ;		// this will also work but it is less explicit
~~~
 We use let again, we then define the variable name, then the datatype, then we initialise it (with lowercase true, false)
 
 #### Arrays 
 Arrays are not a datatype, but a primitive compound type: Primitive datatypes are usually used to describe int, bool, char, float, but we also refer to arrays as a primitive compound type in Rust.
 
 In our game, the array datatype will be used to represent our world, 75 X 75 values. 
 
``` [i32, 9]; - an array of 9 32 bit integers
 	 [i32, 1,2,3,4,5,6,7,8,9]; - we can also initialise it this way 
```
Either way, the size of the array is fixed and it expects it to be filled with values up to the size of the array. 

To make it multi-dimensional, we wrap it in another set of square brackets 
```
[[0u8; 75]; 75]; - a 2D 75 X 75 array 
```

*NB: If you want to specify the datatype, you must initialise the array*

*NB: Arrays are zero-based*

#### Control Structures 
For loops look like this: 
~~~
for i in 0..74 {
	..control loop code here 
}
~~~
the i is a control loop variable, and it is mutable by implication, it is obviously going to change. 

if blocks look like this:

~~~
if <insert_test_here> {
	..insert action here;
} else {
    ..insert action here;
}
~~~

#### Functions 
~~~
fn main() {
	..function code
}
~~~
For all intents and purposes, functions are scope definitions. In most cases, apart from main, we have to give data to a function and return data from a function. 

~~~
fn census(_world: [[0u8; 75]; 75]) -> u16
{
	let mut count = 0;
	for i in 0..74 {
		for j in 0..74 {
			if world[i][j] == 1 
			{
				count += 1;
			}
		}
	}
	count
}
~~~

A few things happening here. The  `-> u16` tells the compiler that this function will return a single unsigned 16 bit integer. You can return basically any type of function. 

Secondly, Rust doesn't use a `return` keyword. You just put the return value at the very end of the function. 

Thirdly, there is no semi colon at the end of the return function

We can return multiple values from a function, in teh form of tuples. Just put them in comma-separated list values like (value1, value 2)

(val1, val2) = myFunc(someData)


You have to declare the input parameter iin the function declaration, including name and datatype. If the parameter passed into the functino when it's called doesn't match the function signature, it will generate an error. 

Pass-by-Value = The value itself is passed into the function - Read-Only, can't make any changes to the data. The variable passed to the function is untouched when the function fonishes executing. 

Pass-by-Reference = The memory location of the data is passed into the function

*NB: In Rust, only one function can ever own a variable. When the variable is passed to the function, it becomes the property of that function* 

*NB: Variable ownership is something I've never heard about before* 

**The last value in a function becomes the returned value!**

**Expressions don't use semicolons to terminate them as statements do!**

#### Tuples 
A tuple is, mathematically speaking, a finite ordered list. 

### Coding out the Game of Life 
So you basically need to create an algorithm to sum all the surrounding boxes, that are either 1 or 0. 

If we were to code this out using a non-clever brute force, we would  have something like: 

- i=0, j=0
	- 3 lines of code 
- i=74, j=0
	- 3 lines of code 
- i=0, j=74
	- 3 lines of code  
- i=74, j=74 -- this is the four corners 
	- 3 lines of code 
- (i > 0 && i< 74) && (j > 0 && j < 74) -- all the middle ones
	- 8 lines of code 
- (i > 0 && i< 74) && (j = 0) - top non corner ones 
	- 5 lines of code 
- (i > 0 && i< 74) && (j = 74) - bottom non corner ones 
	- 5 lines of code 
-  i=0 && (j > 0 && j < 74)- left non corner ones 
	- 5 lines of code   
-  i=75 && (j > 0 && j < 74)- right non corner ones 
	- 5 lines of code   

This would work but we have 9 if/ else if loops woth 40 lines of code. 

But in using this approach, we are starting with zero and trying to *include cases*, i.e. we are thinking exclusively. When what we should be doing is looking for what we can *exclude* and think inclusively. 

So in this case, if we say, well if i > 0, then it will *always* check the one behind it, even if it is on top, bottom, or right. So in most cases, we check behind us

If j > 0, it will *always* check the one above it even if it is on the left, right or bottom.  So in most cases, we check above us. 

Start by thinking what happens *in most cases* rather than what happens *in specific cases*