use std::io::{self, Write};
use rand::Rng;
use std::thread;
use std::time::Duration;

struct InputValues {
    int: Option<i32>,
    float: Option<i64>,
    text: Option<String>,
    bool: Option<bool>,
}

pub fn clear(err_msg: &String) {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().expect(err_msg);
}

pub fn sleep(mili: u64) {
    thread::sleep(Duration::from_millis(mili));
}

pub fn ask(prompt: &String, select: &[String], err_msg: &String) -> i32 {
    clear(err_msg);
    println!("{}\n\n", prompt);
    for (index, value) in select.iter().enumerate() {
        println!("[{}]{}", index, value);
    }
    print!(">> ");
    io::stdout().flush().expect(err_msg);

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(err_msg);

    let input = input.trim();
    let mut input_value = InputValues {
        int: None,
        float: None,
        text: None,
        bool: None,
    };
    
    parse_input(&mut input_value, input.to_string());
    choose_action(&mut input_value, select, err_msg)
}

fn parse_input(input_value: &mut InputValues, input: String) -> &mut InputValues {
    if let Ok(int) = input.parse::<i32>() {
        input_value.int = Some(int);
        return input_value
    }

    if let Ok(float) = input.parse::<i64>() {
        input_value.float = Some(float);
        return input_value
    }

    if let Ok(bool) = input.parse::<bool>() {
        input_value.bool = Some(bool);
        return input_value
    }

    input_value.text = Some(input);
    input_value
}

fn choose_action(input_value: &mut InputValues, select: &[String], err_msg: &String) -> i32 {
    if input_value.int != None {
        if input_value.int >= Some(0) {
            if input_value.int < Some(select.len() as i32) {
                let val: i32 = input_value.int.expect(err_msg);
                return val
            }
        }
    }

    if input_value.float != None {
        if input_value.float >= Some(0) {
            if input_value.float < Some(select.len() as i64) {
                let val64: i64 = input_value.float.unwrap_or(0);
                return val64 as i32
            }
        }
    }

    if input_value.text != None {
        let input_text: String = input_value.text.clone().unwrap_or(select[0].clone()).to_lowercase();
        match select.iter().position(|x| x == &input_text) {
            Some(index) => {return index as i32},
            None => {}
        }
    }

    if input_value.bool == Some(false) {
        return 0
    }

    println!("ew...\nWhich one are you talking about? Well, let's just say I chose this one!");
    sleep(1000);
    let mut rng = rand::thread_rng();
    rng.gen_range(0..select.len() as i32)

}

pub fn trick(prompt: &String, sec: u64, times: i32, err_msg: &String) {
    let colors = ["\x1b[90m", "\x1b[91m", "\x1b[92m", "\x1b[93m", "\x1b[94m", "\x1b[95m", "\x1b[96m"];
    let background = ["\x1b[41m", "\x1b[42m", "\x1b[43m", "\x1b[44m", "\x1b[45m", "\x1b[46m"];
    for _ in 1..times {
        clear(err_msg);
        let mut color_rng = rand::thread_rng();
        let color_selected = color_rng.gen_range(0..colors.len() as i32);
        let mut back_rng = rand::thread_rng();
        let back_selected = back_rng.gen_range(0..background.len() as i32);

        println!(
            "{}{}{}",
            colors[color_selected as usize],
            background[back_selected as usize],
            prompt
        );
        sleep(sec);
    }
    clear(err_msg);
}