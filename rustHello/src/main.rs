// This declaration will look for a file named `struct_Info.rs` or `struct_Info/mod.rs` and will
// insert its contents inside a module named `struct_Info` under this scope
mod struct_Info;
mod front_of_house;
mod NumberClone;
mod TraitSample;
mod GenericFn;
mod ClosuresFun;
mod Lifetime;
mod ReadFile;
mod Ferris;
mod SimpleTypeElement;
mod PrintComplex;
mod Loopp;
mod Strings;
mod Ownership;
mod Boxing;
mod ReferenceCell;
mod TraitObject;
mod blog;
mod Threadss;
mod ThreadChannell;
mod ArraySlice;


mod macro_sample;
mod str_conversion;
mod externallibsample;
mod visibilitytestOuter;
mod entry;
mod matchtest;
mod enum_discriminant_repr;
mod binary_search_sample;
mod formatterr;
mod formatterr_no_std_2;
mod formatterr_no_std_3;
mod formatterr_no_std_4;
mod feature_select_sample;

// ^^ 上面这些 ，只是把本级crate下的mod类包含进来，因为一个crate下的 main.rs 或 lib.rs是汇总文件


pub fn main() {
    entry::main();
}












































































