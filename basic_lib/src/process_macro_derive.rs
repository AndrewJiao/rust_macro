use macro_lib::SuperTrait;

///
/// 这是一个派生宏demo，核心要点是他们必须在单独的包里定义
/// 只能在lib.rs里面定义宏解析函数
/// 要派生的属性和类尽可能用全路径名称。避免派生后找不到该属性和函数
///
#[cfg(test)]
mod test_mod {
    use crate::process_macro_derive::{SomeObj, SomeOtherObj, SuperTrait};

    #[test]
    fn do_test() {
        //会自动派生trait并打印属性
        SomeObj::do_some_echo();
        SomeOtherObj::do_some_echo();

        SomeOtherObj("any".to_string()).do_some_obj_echo();
        SomeObj{ name:"any".to_string() }.do_some_obj_echo();
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



