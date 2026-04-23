#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pose {
    pub x: i32,
    pub y: i32,
    pub heading: char,
}

impl Pose {
    pub fn new(x: i32, y: i32, heading: char) -> Self {
        Pose { x, y, heading }
    }

    /// 沿当前朝向前进一格。
    pub fn forward(&mut self) {
        self.step(1);
    }

    /// 沿当前朝向后退一格。
    pub fn backward(&mut self) {
        self.step(-1);
    }

    /// `step` 为 `1` 表示沿朝向移动一格，`-1` 表示反方向移动一格。
    fn step(&mut self, step: i32) {
        match (self.heading, step) {
            ('E', 1) => self.x += 1,
            ('E', -1) => self.x -= 1,
            ('S', 1) => self.y -= 1,
            ('S', -1) => self.y += 1,
            ('W', 1) => self.x -= 1,
            ('W', -1) => self.x += 1,
            ('N', 1) => self.y += 1,
            ('N', -1) => self.y -= 1,
            _ => (),
        }
    }

    pub fn turn_left(&mut self) {
        self.heading = match self.heading {
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            'N' => 'W',
            _ => self.heading,
        };
    }

    pub fn turn_right(&mut self) {
        self.heading = match self.heading {
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            'N' => 'E',
            _ => self.heading,
        };
    }

    /// 按序执行原子操作；`Forward(±1)` 每次只移动一格，多格由多个 `Forward` 组成。
    pub fn execute(&mut self, actions: &[Action]) {
        for a in actions {
            match *a {
                Action::Forward(1) => self.forward(),
                Action::Forward(-1) => self.backward(),
                Action::Forward(_) => {}
                Action::TurnLeft => self.turn_left(),
                Action::TurnRight => self.turn_right(),
            }
        }
    }
}

/// 可组合的原子动作（与业务指令「生成 / 执行」分离中的执行侧）。
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Action {
    /// 沿当前朝向前进 (`1`) 或后退 (`-1`) 一格。
    Forward(i32),
    TurnLeft,
    TurnRight,
}

impl Default for Pose {
    fn default() -> Self {
        Pose {
            x: 0,
            y: 0,
            heading: 'N',
        }
    }
}
