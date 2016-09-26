fn main() {
    println!("Hello, world!");
	println!("{} days",31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

println!("{} of {:b} , {:b}, {:b}  people", 1,2,3,12);
println!("{number:>width$}", number=32, width=6);
println!("{number:>0width$}", number= 42, width=5);

struct Structure(i32);
Structure{a ,b }


println!("this struct '{}' won't print...", Structure(1));
}
