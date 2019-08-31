extern crate string_interner;
use string_interner::StringInterner;

fn main() {
    println!("hello");

    let mut interner = StringInterner::default();
    let sym0 = interner.get_or_intern("Elephant");
    let sym1 = interner.get_or_intern("Tiger");
    let sym2 = interner.get_or_intern("Horse");
    let sym3 = interner.get_or_intern("Tiger");
    assert_ne!(sym0, sym1);
    assert_ne!(sym0, sym2);
    assert_ne!(sym1, sym2);
    assert_eq!(sym1, sym3); // same!
}