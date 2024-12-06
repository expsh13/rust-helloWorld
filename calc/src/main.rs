use std::io::stdin;

fn main() {
    let mut memory = Memory { slots: vec![] };
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        // 行毎読み込み
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // 空白で分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();

        // メモリへの書き込み
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with("+") {
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            memory.add_and_print(tokens[0], -prev_result);
            continue;
        }

        // 式の計算
        let left: f64 = memory.eval_token(tokens[0]);
        let right: f64 = memory.eval_token(tokens[2]);
        let result = eval_expression(tokens[1], &left, &right);
        print_output(result);

        prev_result = result;
    }
}
fn print_output(value: f64) {
    println!(" => {}", value);
}

struct Memory {
    slots: Vec<(String, f64)>,
}

impl Memory {
    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        // メモリ名を抜き出す
        let slot_name = &token[3..token.len() - 1];
        // 全てのメモリを探索
        for slot in self.slots.iter_mut() {
            if slot.0 == slot_name {
                slot.1 += prev_result;
                print_output(slot.1);
                return;
            }
        }
        // メモリがない場合
        self.slots.push((slot_name.to_string(), prev_result));
        print_output(prev_result);
    }
    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            for slot in &self.slots {
                if slot.0 == slot_name {
                    return slot.1;
                }
            }
            // メモリがない場合初期値
            0.0
        } else {
            token.parse().unwrap()
        }
    }
}

fn eval_expression(operator: &str, left: &f64, right: &f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // コンパイラにここは来ないことを教える
            unreachable!()
        }
    }
}
