use iced::*;
use iced::canvas::*;
use iced::Length::*;
use rand::{Rng, thread_rng};
use crate::algorithms::*;
use crate::visualizer::*;


pub const WIDTH: u16 = 840;
pub const HEIGHT: u16 = 768;
pub const PADDING: u16 = 5;
pub const BAR_WIDTH: f32 = 5.0;

pub const NUM_BARS: usize = ((WIDTH as f32 - PADDING as f32) / (BAR_WIDTH as f32)) as usize;


pub struct Painting {
    canvas_cache: Cache,
    algorithm: IntegerSortingAlgorithm,
    bars: Vec<i32>,
    compared_index: (usize, usize)
}

impl Painting {
    fn generate_bars(sz: usize) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for _ in 0..sz {
            v.push(rng.gen_range(10..700));
        }
        v
    }

    pub fn new(algo: Algorithm) -> Self {
        Painting {
            canvas_cache: Cache::default(),
            bars: Painting::generate_bars(NUM_BARS),
            algorithm: Algorithm::new(algo),
            compared_index: (0, 0),
        }
    }


    pub fn request_redraw(&mut self) {
        self.canvas_cache.clear()
    }

    pub fn sort_step(&mut self) {
        match self.algorithm.sort_step(&mut self.bars) {
            None => {}
            Some(e) => {
                self.compared_index = (e.compared.0.unwrap_or(0), e.compared.1.unwrap_or(0))
            }
        }
        self.canvas_cache.clear();
    }

    pub fn is_sorted(&self) -> bool {
        self.bars.windows(2).all(|w| w[0] <= w[1])
    }

    pub fn view(&mut self) -> Container<Message> {
        let canvas = Canvas::new(self)
            .width(Units(WIDTH))
            .height(Units(HEIGHT));


        Container::new(canvas)
            .max_height(HEIGHT as u32)
            .width(FillPortion(15))
            .height(Units(HEIGHT))
            .padding(Padding::new(PADDING))
            .into()
    }
}


impl<Message> Program<Message> for Painting {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let canvas = self.canvas_cache.draw(bounds.size(), |frame| {
            let height = frame.height();

            let mut x = 0.0;

            let max_height = *self.bars.iter().max().unwrap_or_else(||&1) as f32;

            for (idx, i) in self.bars.iter().enumerate() {
                x += BAR_WIDTH;

                let stroke: Stroke;

                if idx == self.compared_index.0 || idx == self.compared_index.1 {
                    stroke = Stroke {
                        width: BAR_WIDTH,
                        color: Color::new(0.0, 1.0, 0.0, 1.0),

                        ..Stroke::default()
                    };
                } else {
                    let bar_height = self.bars[idx] as f32;
                    stroke = Stroke {
                        width: BAR_WIDTH,
                        color: Color::new(bar_height/max_height, 0., 1.0-bar_height/max_height, 1.0),
                        ..Stroke::default()
                    };
                }

                let bar =
                    Path::line(Point::new(x, height), Point::new(x, height - *i as f32));

                frame.stroke(&bar, stroke);
            }
        });

        vec![canvas]
    }
}
