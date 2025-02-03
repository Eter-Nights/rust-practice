use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use piston_window::rectangle::Shape;
use piston_window::types::Color;
use piston_window::{Context, G2d, Key};
use rand::Rng;
/// 食物颜色
const FOOD_COLOR: Color = [255.0, 0.0, 255.0, 1.0];
/// 上边框颜色
const T_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 下边框颜色
const B_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 左边框颜色
const L_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];
/// 右边框颜色
const R_BORDER_COLOR: Color = [0.0000, 0.5, 0.5, 0.6];

///游戏结束颜色
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

/// 移动周期，每过多长时间进行一次移动
const MOVING_PERIOD: f64 = 0.3;

pub struct Game {
    /// 蛇的主体
    snake: Snake,
    /// 食物是否存在
    food_exists: bool,
    /// 食物x坐标
    food_x: i32,
    /// 食物y坐标
    food_y: i32,
    /// 游戏的宽
    width: i32,
    /// 游戏的高
    height: i32,
    /// 游戏是否结束
    game_over: bool,
    /// 等待时间
    waiting_time: f64,
    /// 是否暂停
    game_pause: bool,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            food_exists: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            game_pause: false,
        }
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);
        if self.food_exists {
            draw_block(
                FOOD_COLOR,
                Shape::Round(8.0, 16),
                self.food_x,
                self.food_y,
                con,
                g,
            );
        }

        //上边框
        draw_rectangle(T_BORDER_COLOR, 0, 0, self.width, 1, con, g);
        // 下边框
        draw_rectangle(B_BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        // 左边框
        draw_rectangle(L_BORDER_COLOR, 0, 1, 1, self.height - 2, con, g);
        // 右边框
        draw_rectangle(
            R_BORDER_COLOR,
            self.width - 1,
            1,
            1,
            self.height - 2,
            con,
            g,
        );

        // 如果游戏失败 绘制游戏失败画面
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        // 输入R重启游戏
        if key == Key::R {
            self.restart()
        }

        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::P => {
                // 输入 P 暂停/启动游戏
                self.game_pause = !self.game_pause;
                None
            }
            _ => None,
        };

        if let Some(d) = dir {
            // 如果输入方向为当前方向的相反方向，不做任何处理
            if d == self.snake.head_direction().opposite() {
                return;
            }
        }

        // 如果为有效输入，直接刷新蛇的方向
        self.update_snake(dir);
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.game_pause || self.game_over {
            return;
        }

        self.waiting_time += delta_time;

        if !self.food_exists {
            self.add_food()
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }
    /// 更新蛇的数据
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.game_pause {
            return;
        }
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    /// 检查当前游戏蛇的生存状态，蛇自身碰撞检测、游戏边界碰撞检测
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        !self.snake.over_tail(next_x, next_y) &&
            next_x > 0 &&
            next_y > 0 &&
            next_x < self.width - 1 &&
            next_y < self.height - 1
    }

    /// 添加果子
    fn add_food(&mut self) {
        let mut rng = rand::rng();

        let mut new_x = rng.random_range(1..self.width - 1);
        let mut new_y = rng.random_range(1..self.height - 1);

        while self.snake.over_tail(new_x, new_y) {
            new_x = rng.random_range(1..self.width - 1);
            new_y = rng.random_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    /// 是否吃到了果子
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
        self.game_pause = false;
    }
}
