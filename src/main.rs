enum Hand {
    Rock,
    Scissors,
    Paper,
}
impl From<usize> for Hand {
    fn from(from: usize) -> Hand {
        match from {
            0 => Hand::Rock,
            1 => Hand::Scissors,
            2 => Hand::Paper,
            _ => unreachable!(),
        }
    }
}
impl From<Hand> for usize {
    fn from(from: Hand) -> usize {
        match from {
            Hand::Rock => 0,
            Hand::Scissors => 1,
            Hand::Paper => 2,
        }
    }
}
impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Hand::Rock => "グー",
                Hand::Scissors => "チョキ",
                Hand::Paper => "パー",
            }
        )
    }
}

#[derive(Clone)]
enum Result {
    Win,
    Lose,
    Draw,
}
impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Result::Win => "勝ち",
                Result::Lose => "負け",
                Result::Draw => "引き分け",
            }
        )
    }
}

fn judgement(my: impl Into<usize>, target: impl Into<usize>) -> Result {
    let result: Vec<Vec<Result>> = vec![
        vec![Result::Draw, Result::Win, Result::Lose],
        vec![Result::Lose, Result::Draw, Result::Win],
        vec![Result::Win, Result::Lose, Result::Draw],
    ];
    return result[my.into()][target.into()].clone();
}

fn input() -> usize {
    let mut guess: String = "".to_string();
    let _ = std::io::stdin().read_line(&mut guess).ok();
    let answer = guess.trim().parse::<u32>().ok().unwrap_or(0);
    answer as usize
}
fn input_y_or_n() -> char {
    let mut guess: String = "".to_string();
    let _ = std::io::stdin().read_line(&mut guess).ok();
    let answer = guess.trim().parse::<char>().ok().unwrap_or('y');
    answer
}

fn main() {
    println!("じゃんけんを始めます。");
    loop {
        let target: Hand = (rand::random::<usize>() % 3).into();
        println!("相手の手の内: {}", target);
        println!("じゃんけん！？[グー0, チョキ1, パー2]");
        println!("{}！", judgement(input(), target));
        {
            println!("じゃんけんを続けますか？[y/N]");
            if input_y_or_n() == 'N' {
                break;
            }
        }
    }
    println!("じゃんけんを終わります。");
}
