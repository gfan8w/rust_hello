macro_rules! make_public {
    // use vis type for visibility keyword and ident for struct name
    (
        // meta data about struct
        $(#[$meta: meta])*
        $vis: vis struct $struct_name:ident
        {
            $(
                // meta data about field
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_type:ty
            ) , * $(,)+
        }
    ) => {
        {
            $(#[$meta])*
            pub struct $struct_name
            {
                $(
                $(#[$field_meta])*
                  pub $field_name : $field_type ,
                )*
            }
        }
    };
}

fn main(){
    make_public!{
        #[derive(Debug)]
        struct Name{
            n:i64,
            t:i64,
            g:i64,
        }
    }

    expand();
}




