
fn main() {}

#[cfg(test)]
mod test_model {
    use basic_lib::process_macro_derive::{SomeObj, SomeOtherObj, SuperTrait};
    //注意，派生宏的trait派生定义和trait都需要引入 如这个SuperTrait
    use macro_lib::SuperTrait;

    #[test]
    fn test_process_macro(){

    }

    ///
    /// 注意依赖的关系
    /// entry_main
    /// | basic_lib:SuperTrait
    /// | | macro_lib:SuperTrait_macro
    ///
    #[test]
    fn macro_simple_derive_trait_fn() {
        // 测试引入使用了macro派生宏的结构体
        SomeObj::do_some_echo();
        SomeOtherObj::do_some_echo();

        SomeOtherObj("any".to_string()).do_some_obj_echo();
        SomeObj {
            name: "any".to_string()
        }.do_some_obj_echo();

        // 测试本地结构体使用派生宏，手动实现的trait派生宏需要单独引入派生宏中的定义的trait，
        // 如macro_lib::SuperTrait,而不是直接引入特性SuperTrait
        MainObj.do_some_obj_echo();
        MainObj::do_some_echo()
    }

    #[derive(SuperTrait)]
    pub struct MainObj;
}

