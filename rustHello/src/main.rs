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
mod Box_sample;
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
mod is_copy;
mod refutable_sample;
mod vec_reallocate_immutable_address_change;
mod Rc_sample;
mod RefCell_sample;
mod Deref_sample;
mod pointer_stack;
mod show_type_size;
mod string_str;
mod global_static;
mod generics_constraint;
mod array_2d_sample;
mod raw_pointer;
mod adhoc_polymorphism;

// ^^ 上面这些 ，只是把本级crate下的mod类包含进来，因为一个crate下的 main.rs 或 lib.rs是汇总文件


pub fn main() {
    entry::main();
}












































































