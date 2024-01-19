//<editor-fold desc="过程宏 ">
///
/// 类属性宏
/// 模拟定义的过程宏，实现原本方法的拓展
/// 本测试同故宫get_method类属性宏实现解析宏上的属性识别路由path和请求方式method
///
#[cfg(test)]
mod test_attribute {
    use macro_lib::get_method;

    //get attr
    #[test]
    pub fn derive_attribute() {
        test_get("test");
        test_value_2(32,23.0);
        test_no_param();
    }

    #[get_method(method = "GET", path = "/api/v1/no_param")]
    fn test_no_param() {
        println!("==== basic method and no param");
    }

    #[get_method(method = "POST", path = "/api/v1/test")]
    pub fn test_get(const_param: &str) {
        println!("==== basic method and param = {}", const_param);
    }

    #[get_method(method = "GET", path = "/api/v1")]
    pub fn test_value_2(value: i32, value2: f32) {
        println!("==== just test 2");
    }
}

//</editor-fold>
