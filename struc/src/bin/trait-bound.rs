// 進行度をあらわす Progress 構造体
struct Progress<T> {
    start: T,   // 始点
    end: T,     // 終点
    step: T,    // 増分
    current: T, // 現在位置
}

impl<T> Progress<T> {
    fn foward(&mut self) {
        self.current += self.step;
    }
}

fn main() {
    let progress = Progress {
        start: 0,
        end: 100,
        step: 10,
        current: 0,
    };
}
