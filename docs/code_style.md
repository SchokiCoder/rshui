# Inspiration

The code style's inspiration is the suckless style for C.  
However, it being for C and this project being written in Rust, causes some icky
adjustments being necessary.  

# Variable declarations

## Type inference

Rust unlike C has type inference that means declaring a variable without type
info is fine if the compiler can infer it.  

```Rust
fn main()
{
    let foo = true;
    let bar = HideCursor::from(std::io::stdout().into_raw_mode().unwrap());
}
```

Guess what the type of "bar" is.  
"foo" is fine but "bar" gets sucky, which can hurt transparency.  
I discourage it. When in doubt, give the type.  
Btw the type of "bar" is
`termion::cursor::HideCursor<termion::raw::RawTerminal<std::io::Stdout>>`.

## Redeclaration

Suckless C declares variables at the beginning of a function and forbids mixing
them with code.  
So far so good but Rust allows seamless redeclaration of a variable:

```Rust
fn main()
{
	let _args = std::env::args();

	let _args = true;

	let _args = "trolled";
}
```

This can be used if you want a variable to change at just one point and forbid
mutation elsewhere.  

```Rust
fn main()
{
	let mut donuts_delivered: bool = false;
	let important_flag: bool = false;

	for _ in 0..10 {
		// one point where we want to change this otherwise immutable variable
		if donuts_delivered && rand::random::<i32>() <= 9000 {
			let important_flag = true;
		}

		// buncha stuff...

		if rand::random::<i32>() > 420 {
			donuts_delivered = true;
		}

		// buncha stuff...

		println!("flag: {}", important_flag);
	}
}
```

It is kind of error prone though.  
If you have a typo in the redeclaration, you get a warning about an unused
variable and thats it.  
It's maybe best to just forbid redeclaration?

# Switch vs Match

The suckless C style formats a `switch` like this:

```C
int main(int argc, char **argv)
{
	switch (argc) {
	case 0:
		fprintf(stderr, "no arguments?\n");
		return -1;
		break;
	
	case 4:
		fprintf(stderr, "too many args\n");
		return -1;
		break;
		
	default:
		printf("All fine :)\n");
	}
}
```

A `case` is on the same level as the switch.  
This if fine for C but applying it to Rust's `match` is odd, because Rust's
branches need brackets and at the end of the `match`, there will be two
brackets on the same level next to each other.  

```Rust
fn main()
{
	let args = std::env::args();

	match args.len() {
	0 => {
		panic!("no args?");
	}

	4 => {
		panic!("too many args");
	}

	_ => {
		println!("this fine :)");
	}
	}
}
```

So why not just raise the bracket of the case?  
This would only have two brackets next to each other if the last case needs a
bracket.  
For me it turns out that this inconsistency lets my brain hiccup, once i read
that. So from now on, put the brackets right smack dab next to each other on the
same line.  

```Rust
return match funny_number {
69 => {
	print!("...are you entertained yet?");
}

43 => {
	print!("Don't worry i will not go as far as Thorium did.");
}}
```

# Function nesting

I just found out that Rust can even do that.  
This sucks, I keep brackets on own line and discourage function nesting.  
Why discourage? Why not?  
Why would you want to nest functions?  
Optimally functions aren't longer than 25 to 50 lines, given that context there
is no room for a function within the function.  

# Headers / Modules

In Rust we use `use` and actual code import happen via `mod` but that only
matters for local files.  
In the file `mod`'s are listed before any `use`'s.  
Both are to be done in alphabetical order with `crate::` modules first,
other workspace modules second and external modules last.  
Seperate `use` of external modules and workspace modules with a newline.  

# other Rust stuff

Rust has various features which have no equivalent in C.  
I am not yet entirely sure how to handle these things:  

- Namespaces
- Closures
- Struct functions
- Traits
- Lifetimes
- and probably more

I may come up with something as i go.  

# Dropped from C suckless

Suckless code style describes certain practices which can not be used in Rust
for one reason or another.  
The following has been dropped from
[suckless code style](http://suckless.org/coding_style/):  

- "C Features" except "Do not mix declarations and code" (NA)
- "Functions" "Function name and argument list on next line."
  (for grep just use `grep fn funcname` instead)
- "Variables"
  "Global variables not used outside translation unit should be declared static."
- "Keywords"
	- "Preferably use () with sizeof." (NA)
	- "Do not use a space with sizeof()." (NA)
- "Switch" "Comment cases that FALLTHROUGH." (NA)
- "User Defined Types" "Typedef opaque structs." (NA)
- "Tests and Boolean Values" (NA)
- "Handling Errors" (too different in general)
- "Enums and #define" (NA)
