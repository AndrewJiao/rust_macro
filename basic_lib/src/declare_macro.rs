///
/// 申明宏
/// 匹配的类型
/// item ——一个项（item），像一个函数，结构体，模块等。
/// block ——一个块 （block）（即一个语句块或一个表达式，由花括号所包围）
/// stmt —— 一个语句（statement）
/// pat ——一个模式（pattern）
/// expr —— 一个表达式（expression）
/// ty ——一个类型（type）
/// ident—— 一个标识符（indentfier）
/// path —— 一个路径（path）（例如，foo，::std::mem::replace，transmute::<_, int>，...）
/// meta —— 一个元数据项；位于#[...]和#![...]属性
/// tt——一个词法树
/// vis——一个可能为空的Visibility限定词
///


///eg.
#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};

    ///
    /// 定义一个加法运算
    ///
    macro_rules! my_add {
        ($($arg:ident),*) => {
            0$(+$arg)*
        };
    }
    #[test]
    fn run_test_basic() {
        let value = my_add!();
        println!("value = {}", value)
    }
    ///
    /// 关于类型
    ///
    macro_rules! type_ident {
        ($arg:ident) => {$arg};
    }

    ///eg for item
    #[derive(Debug)]
    struct A;

    fn test_fun_a() {
        println!("from fun a")
    }

    mod a_mod {}


    ///
    #[test]
    fn about_type_indent() {
        println!("type indent strut A = {:?}", type_ident!(A));

        let test_value = "test_str";
        println!("type indent value = {:?}", type_ident!(test_value));

        let a_function = type_ident!(test_fun_a);
        a_function();
    }


    macro_rules! type_meta {
        ($arg:meta) => {$arg};
    }


    #[derive(Debug)]
    struct MetaA;

    #[test]
    fn about_type_meta() {
        // let x = type_meta!(MetaA);
        // let x = type_meta!(A);
        // let mut value_a = "str";
        // let x = type_meta!(value_a);
        // let x = type_meta!(x);
    }


    macro_rules! type_tt {
        ($arg:tt) => {$arg};
    }

    ///
    /// note:宏可以多个匹配，以便编译器能识别需要哪一个
    /// note:tt语法树是作为token接收参数，可以不需要，号
    /// note:token之间作为参数传递可以不用，号分割,编译器可以自动识别
    /// 宏可以递归执行
    ///
    macro_rules! my_multi_add {
        //匹配单个
        ($a:expr) => {
            $a
        };

        //匹配两个数相加
        ($a:expr,$b:expr) => {
            $a+$b
        };

        //匹配多个
        ($a:expr,$($b:tt)*) => {
            $a+ my_multi_add!($( $b )*)
        };

    }
    #[test]
    fn about_type_tt() {
        let a = type_tt!(A);
        let result_value = my_multi_add!(1,2,3,4,5,6);
        println!("result_value = {}", result_value);
    }


    macro_rules! type_ty {
        ($input_arg:expr, $arg:ty) => {
            $input_arg as $arg
        };
    }

    #[test]
    fn about_type_ty() {
        let value = type_ty!(12, f64);
        println!("value string as f64 {}", value)
    }


    macro_rules! type_expr {
        ($arg:expr) => {$arg};
    }

    macro_rules! my_judge {
        ($ex:expr,$arg1:tt,$arg2:tt) => {
            if $ex {
                format!("sub_if_{}",$arg1)
            }else {
                format!("sub_else_{}",$arg2)
            }
        };
    }

    #[test]
    fn about_type_expr() {
        let x = type_expr!(if 1==1 {"true"} else{ "false"});
        println!("{:?}", x);
        println!("value = {}", x);
        println!("my judge = {}", my_judge!(1==1,"true","false"))
    }


    ///
    ///宏的内部规则
    ///note:宏可以定义内部方法，给与宏内部调用
    ///
    // macro_rules! internal_rule_test {
    //     (@inner $method:ident,$($args:tt)*)=>{
    //
    //         match $method($($args)*){
    //             Ok(value) => value,
    //             Err(err) =>  {
    //                 return Err(err)
    //             }
    //         }
    //     };
    //     ($method:ident($($args:tt)*))=>{
    //         internal_rule_test!(@inner $method,$($args)*)
    //     };
    // }
    fn some_work(i: i64, j: i64) -> Result<(i64, i64), String> {
        if i + j > 2 {
            Ok((i, j))
        } else {
            Err("error".to_owned())
        }
    }

    #[test]
    fn internal_rule() {
        // let value = internal_rule_test!(some_work(1, 3));
        // println!("value = {:?}", value);
        //
        // let value1 = internal_rule_test!(some_work(0, 1));
        // println!("value1 = {:?}", value1);
    }


    ///
    ///note:尝试解析整个结构体
    ///

    macro_rules! analyze_struct {
        (
            $(#[$meta:meta])*
            $vi:vis struct $struct_name:ident{
                $(
                    $(#[$filed_meta:meta])*
                    $filed_vis:vis $filed_name:ident: $filed_type:ty
                ),*
            }
        ) => {
            $(
                println!("meta = {:?}",stringify!($meta));
            )*
            println!("vis = {:?},struct name = {:?}", stringify!($vi), stringify!($struct_name));
            $(
                $(
                    println!("filed_meta = {:?}",stringify!($filed_meta));
                )*
                println!("filed_vis = {},filed_name = {:?}, filed_type = {:?}", stringify!($filed_vis),stringify!($filed_name) ,stringify!($filed_type));
            )*
        };
    }

    #[test]
    fn analyze_struct() {
        analyze_struct!(
            #[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
            pub struct ToBeAnalyze {
                #[just_test]
                pub filed1: u32,
                #[特殊]
                pub filed2: String
            }
        );
    }

}
