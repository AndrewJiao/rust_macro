mod process_macro;
mod declare_macro;

use macro_lib::SuperTrait;

#[cfg(test)]
mod test_mod {
    use crate::{SomeObj, SomeOtherObj, SuperTrait};

    #[test]
    fn do_test() {
        SomeObj::do_some_echo();
        SomeOtherObj::do_some_echo();

        SomeOtherObj("any".to_string()).do_some_obj_echo();
        SomeObj{
            name:"any".to_string()
        }.do_some_obj_echo();
    }


}

pub trait SuperTrait {
    fn do_some_echo();
    fn do_some_obj_echo(&self);
}


#[derive(Debug, SuperTrait)]
pub struct SomeObj {
    pub name: String,
}


#[derive(SuperTrait, Debug)]
pub struct SomeOtherObj(pub String);
