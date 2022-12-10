mod copy_clone_trait;
mod copy_drop_trait;
mod Deref_sample0;
mod Deref_sample1;
mod Deref_sample2;
mod sized_unsized_sample;
mod send_sync_trait;
pub mod PrintComplex;
mod from_into_trait;
mod asref_asmut_trait;
mod default_trait;
mod index_trait;



///重要的trait，必须学会
pub fn run(){
    Deref_sample1::run();

    // 复杂对象如何显示
    PrintComplex::run();
}
