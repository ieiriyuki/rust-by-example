fn main(){

	// formatted print
	// {}: stringified
	println!("{} days", 31);

	// positional arguments
	println!("{0}, this is {1}. {1}, thisis {0}", "Alice", "Bob");

	// named arguments
	println!("{subject} {verb} {object}",
			object="the lazy dog",
			subject="the quick brown fox",
			verb="jumps over");

	// special format
	println!("{} ob {:b} people know binary, the other half doesn't",
			1, 2);

    // right-align text
	println!("{number:>width$}", number=1, width=6);

    // align and pad with 0
	println!("{number:>0width$}", number=1, width=6);

    // fix a code below
	// println!("My name is {0}, {1} {0}", "Bond");
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure wich contains an 'i32'. name it 'Structure'
	#[allow(dead_code)]
	struct Structure(i32);

    // a code below won't work
	// println!("This struct `{}` won't print...", Structure(3));


    // Activity
    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}
