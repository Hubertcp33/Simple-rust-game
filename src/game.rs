use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};
// use crate::draw::draw_block;
// use crate::draw::draw_rectangle;

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    // 蛇
    snake: Snake,

    // 食物是否存在
    foot_exists: bool,
    // 食物的x坐标
    foot_x: i32,
    // 食物的y坐标
    foot_y: i32,

    // 游戏区域的宽度
    width: i32,
    // 游戏区域的高度
    height: i32,

    // 游戏是否结束
    game_over: bool,
    // 等待时间
    waiting_time: f64,
}

impl Game {
    // 创建一个新的游戏
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            foot_exists: true,
            foot_x: 6,
            foot_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    // 处理按键事件
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // 绘制游戏
    pub fn draw(&self, con: &Context, g: &mut G2d){
        self.snake.draw(con, g);

        if self.foot_exists {
            draw_block(FOOD_COLOR, self.foot_x, self.foot_y, con, g);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1,self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);
        
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    // 更新游戏状态
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time; 

        if self.game_over {

            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
            
        }

        if !self.foot_exists {
            self.add_food();
            return;
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // 检查蛇是否吃到食物
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.foot_exists && self.foot_x == head_x && self.foot_y == head_y {
            self.foot_exists = false;
            self.snake.restore_tail();
        }
    }

    // 检查蛇是否存活
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_x < self.width - 1 && next_y > 0 && next_y < self.height - 1
    }

    // 添加食物
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.foot_x = new_x;
        self.foot_y = new_y;
        self.foot_exists = true;
    
    }

    // 更新蛇的位置
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.waiting_time = 0.0;
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        
        }
        self.waiting_time = 0.0;
    }

    // 重新开始游戏
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.foot_exists = true;
        self.foot_x = 6;
        self.foot_y = 4;
        self.game_over = false;

    
    }
}