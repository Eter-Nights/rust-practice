use crate::draw::draw_block;
use piston_window::rectangle::Shape;
use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

/// 蛇身体的颜色
const SNAKE_BODY_COLOR: Color = [0.5, 0.0, 0.0, 1.0];
/// 蛇头的颜色
const SNAKE_HEAD_COLOR: Color = [1.0, 0.00, 0.00, 1.0];


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// 方向输入合法性验证，不能直接转向相反方向
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

/// 块，蛇的身体的最小单元
#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Snake {
    /// 当前朝向
    direction: Direction,
    /// 蛇的身体
    body: LinkedList<Block>,
    /// 蛇的尾巴
    tail: Option<Block>,
}

impl Snake {
    /// 蛇的初始化
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    /// 蛇的绘制
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let mut is_head = true;
        for block in &self.body {
            if is_head {
                is_head = false;
                draw_block(SNAKE_HEAD_COLOR, Shape::Round(10.0, 16), block.x, block.y, con, g);
            } else {
                draw_block(SNAKE_BODY_COLOR, Shape::Round(12.5, 16), block.x, block.y, con, g);
            }
        }
    }

    /// 蛇头的当前坐标
    pub fn head_position(&self) -> (i32, i32) {
        let head = self.body.front().unwrap();
        (head.x, head.y)
    }

    /// 蛇头的当前方向
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    /// 蛇头的下一个位置的坐标
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut moving_dir = self.direction;
        match dir {
            Some(d) => moving_dir = d,
            None => ()
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    /// 向前移动
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => ()
        }

        let (x, y) = self.next_head(dir);
        self.body.push_front(Block { x, y });
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    /// 增加蛇的长度
    pub fn restore_tail(&mut self) {
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk);
    }

    /// 自身碰撞检测
    pub fn over_tail(&self, x: i32, y: i32) -> bool {
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }
        }
        false
    }
}
