use std::io::{self, stdin, Write};

pub fn read_command(prompt: &str) -> (String, Option<Vec<String>>) {
    let mut input = String::new();

    // 打印提示符并等待用户输入
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // 刷新输出缓冲区以确保提示符立即显示
                                   //
                                   // 读取一行输入
    stdin().read_line(&mut input).expect("Failed to read line");

    // 读取一行输入,去除前后空格
    input = input.trim().to_string();
    if let Some(index) = input.find(' ') {
        // 查找第一个空格的位置
        let command = input[..index].to_string(); // 将空格前的字符串作为命令
        let args = input[index + 1..]
            .split_whitespace() // 将空格后的字符串切分为参数
            .map(|s| s.to_string()) // 将字符串切片转换为String
            .collect();
        (command, Some(args))
    } else {
        (input, None)
    }
}

// pub fn get_player_input() -> PlayerStatus {}
