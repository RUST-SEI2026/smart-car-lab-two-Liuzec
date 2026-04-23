use crate::pose::Action;

/// 与倒车 / 加速等业务状态相关的可切换位。
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct State {
    pub is_reverse: bool,
    pub is_fast: bool,
}

impl State {
    pub fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }

    pub fn be_fast(&mut self) {
        self.is_fast = !self.is_fast;
    }

    /// 由当前状态与 `M`/`L`/`R` 之一生成待执行动作序列（不含 `B`/`F` 切换本身）。
    pub fn assemble(&self, cmd: char) -> Vec<Action> {
        match cmd {
            'M' => self.move_assemble(),
            'L' => self.turn_left_assemble(),
            'R' => self.turn_right_assemble(),
            _ => vec![],
        }
    }

    fn move_assemble(&self) -> Vec<Action> {
        let step: i32 = if self.is_reverse { -1 } else { 1 };
        let n = if self.is_fast { 2 } else { 1 };
        vec![Action::Forward(step); n]
    }

    fn turn_left_assemble(&self) -> Vec<Action> {
        let step: i32 = if self.is_reverse { -1 } else { 1 };
        let turn = if self.is_reverse {
            Action::TurnRight
        } else {
            Action::TurnLeft
        };
        if self.is_fast {
            vec![Action::Forward(step), turn]
        } else {
            vec![turn]
        }
    }

    fn turn_right_assemble(&self) -> Vec<Action> {
        let step: i32 = if self.is_reverse { -1 } else { 1 };
        let turn = if self.is_reverse {
            Action::TurnLeft
        } else {
            Action::TurnRight
        };
        if self.is_fast {
            vec![Action::Forward(step), turn]
        } else {
            vec![turn]
        }
    }
}
