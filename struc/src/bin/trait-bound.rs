use std::ops::AddAssign;

// 進行度をあらわす Progress 構造体
struct Progress<T> {
    start: T,   // 始点
    end: T,     // 終点
    step: T,    // 増分
    current: T, // 現在位置
}

// impl<T> Progress<T> {
//     fn foward(&mut self) {
//         self.current += self.step;
//     }
// }

impl<T: AddAssign + Copy> Progress<T> {
    fn foward(&mut self) {
        self.current += self.step;
    }
}

fn main() {
    let mut progress = Progress {
        start: 0,
        end: 100,
        step: 10,
        current: 0,
    };

    while progress.current < progress.end {
        progress.foward();
        println!("Current progress: {}", progress.current);
    }

    for i in (progress.start..=progress.end).step_by(progress.step as usize) {
        println!("Progress: {}%", i);
    }
}
