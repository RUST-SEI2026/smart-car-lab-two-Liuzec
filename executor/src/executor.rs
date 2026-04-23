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

/// 与倒车 / 加速等业务状态相关的可切换位。
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub is_reverse: bool,
}

impl State {
    pub fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }
}

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: State::default(),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'M' => {
                    if self.state.is_reverse {
                        self.pose.backward();
                    } else {
                        self.pose.forward();
                    }
                }
                'L' => {
                    if self.state.is_reverse {
                        self.pose.turn_right();
                    } else {
                        self.pose.turn_left();
                    }
                }
                'R' => {
                    if self.state.is_reverse {
                        self.pose.turn_left();
                    } else {
                        self.pose.turn_right();
                    }
                }
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
