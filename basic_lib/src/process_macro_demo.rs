///
/// 定义一个过程宏
/// 需求：
/// 提供一个注册方法，可以注册函数。
/// 再提供一个方法，接收任何参数，当提供的参数符合任何之前所注册的函数调用条件（参数类似匹配）则调用该函数
///

/**
 * 定义一个调用方法,调用注册的函数
 */
fn do_some<T>(arg: T) -> () {}

///
/// 用于封装注册的函数未统一的形式
///



