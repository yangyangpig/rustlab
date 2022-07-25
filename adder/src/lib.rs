pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }
    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2));
    }
}

// cargo test 命了使用
// cargo test -- --test-threads=1 // 单线程执行
// cargo test -- --show-output // 输出println!输出内容
// 按照名称运行测试的子集，将测试的名称作为cargo test的参数

// 继承测试，在src同级别的文件夹下面，创建test文件夹，运行cargo test --test 文件名，就会在test文件夹下运行
// 与文件名同名的所有测试代码


// rust提供了二进制程序关注点分离的指导性原则
// 将程序拆分为main.rs和lib.rs，将业务逻辑放入lib.rs
// 当命令行拆解逻辑较少时，将它放在main.rs也行
// 当命令行解析逻辑变复杂时候，需要将它从main.rs提取到lib.rs
// 留在main功能有
// 使用参数值调用命令行解析逻辑
// 进行其它配置
// 调用lib.rs中的run函数
// 处理run函数可能出现的错误

//