use winit::{
    event::{Event, WindowEvent, VirtualKeyCode, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi,
};
use pixels::{Error, Pixels, SurfaceTexture};
use rand::Rng;
use rand;
use rusttype;

use std::time;
use std::collections::{VecDeque, HashMap};

trait PixelExt {
    fn access_pixel(&mut self, x: usize, y: usize) -> Option<&mut [u8]>;

    // (height, width)
    fn get_size(&self) -> (u32, u32);
}

impl PixelExt for Pixels {
    fn access_pixel(&mut self, x: usize, y: usize) -> Option<&mut [u8]> {
        let width = &self.context().texture_extent.width;
        let index = (y*(*width as usize) + x)*4;
        self.get_frame().get_mut(index..(index+&4))
    }

    fn get_size(&self) -> (u32, u32) {
        let width  = self.context().texture_extent.width;
        let height = self.context().texture_extent.height;

        (width, height)
    }
}

/// these are game-cell positions. They don't correspond to the
/// position in the window.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Pos<T> {
    x: T,
    y: T,
}

#[derive(Clone, Copy)]
struct World {
    pub width: u32,
    pub height: u32,
    pub block_size: u32,
    pub food_pos: Pos<i32>,
}

impl World {
    fn new(width: u32, height: u32, block_size: u32) -> Self {
        let mut w = Self {
            width,
            height,
            block_size,
            food_pos: Pos { x: 0, y: 0 },
        };

        w.place_food(&[]);

        w
    }

    fn place_food(&mut self, blacklist: &[Pos<i32>]) {
        loop {
            let width = self.width as i32 / 2;
            let height = self.height as i32 /2;

            self.food_pos.x = rand::thread_rng().gen_range(-width..width);
            self.food_pos.y = rand::thread_rng().gen_range(-height..height);

            if blacklist.contains(&self.food_pos) {
                continue
            } else {
                break
            }
        }
    }

    fn draw(&self, pixels: &mut Pixels) {
        let grid_color: [u8; 4] = [0xff; 4];
        let food_color: [u8; 4] = [0xff, 0x00, 0x00, 0x00];

        let (win_width, win_height) = pixels.get_size();
        let win_width  = win_width as i32;
        let win_height = win_height as i32;
        
        // draw the border centered on the window
        let bottom = (self.height*self.block_size/2) as i32 + (win_height/2) as i32 + 2;
        let right = (self.width*self.block_size/2) as i32 + (win_width/2) as i32 - 2;

        // minus one for alignment 
        let top  = (win_height/2) as i32 - (self.height*self.block_size/2) as i32 + 2;
        let left = (win_width/2) as i32 - (self.width*self.block_size/2) as i32 - 2;

        if  !(0..win_width).contains(&right) | 
            !(0..win_width).contains(&left)  |
            !(0..win_height).contains(&top)  |
            !(0..win_height).contains(&bottom)
            { return }

        // draw top and bottom borders
        for x in left..right {
            pixels
                .access_pixel(x as usize, top as usize)
                .and_then::<&mut [u8],_>(|f| {
                    f.copy_from_slice(&grid_color);
                    None
                });

            pixels
                .access_pixel(x as usize, bottom as usize )
                .and_then::<&mut [u8],_>(|f| {
                    f.copy_from_slice(&grid_color);
                    None
                });
        }

        // draw left and right borders
        for y in top..bottom {
            pixels
                .access_pixel(left as usize, y as usize)
                .and_then::<&mut [u8],_>(|f| {
                    f.copy_from_slice(&grid_color);
                    None
                });

            pixels
                .access_pixel(right as usize, y as usize)
                .and_then::<&mut [u8],_>(|f| {
                    f.copy_from_slice(&grid_color);
                    None
                });
        }

        // draw food
        let food_pos = Pos::<u32> {
            x: ((self.food_pos.x*self.block_size as i32) + (win_width/2)  as i32) as u32,
            y: ((-self.food_pos.y*self.block_size as i32) + (win_height/2) as i32) as u32,
        };
        for x in 0..self.block_size-3 {
            for y in 0..self.block_size-3 {
                pixels
                    .access_pixel((food_pos.x + x) as usize, (food_pos.y - y) as usize)
                    .and_then::<Option<&[u8]>,_>(|f| {
                        f.copy_from_slice(&food_color);
                        None
                    });
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Direction { Up, Down, Left, Right }

struct SlitherySnek {
    body: VecDeque<Pos<i32>>,
    dir: Direction,
    world: World,
    grow_buf: u32,
    initial_size: u32,
}

impl SlitherySnek {
    fn new(len: u32, world: World) -> Self {
        let mut snek = Self {
            body: VecDeque::new(),
            dir: Direction::Right,
            world,
            grow_buf: 0,
            initial_size: len,
        };

        for i in 0..len {
            snek.body.push_front( Pos { x: i as i32, y: 0});
        }

        snek
    }

    fn reset(&mut self) {
        self.body.clear();
        self.grow_buf = 0;
        self.dir = Direction::Right;

        for i in 0..self.initial_size {
            self.body.push_front( Pos { x: i as i32, y: 0});
        }
    }

    fn check_dead(&self) -> bool {
        // is it out of bounds?
        let width = self.world.width as i32 /2;
        let height = self.world.height as i32 / 2;

        if !(-width..width).contains(&self.body[0].x) |
            !(-height..height).contains(&self.body[0].y) {
            return true
        }

        // is the body intersecting with itself?
        for i in self.body.iter().skip(1)   {
            if *i == self.body[0] {
                return true
            }
        }

        false
    }

    /// if dir is None, slither in the current direction
    fn slither(&mut self, dir: Option<Direction>) {
        self.dir = match dir {
            Some(Direction::Up)    => if self.dir == Direction::Down  { self.dir } else { Direction::Up    },
            Some(Direction::Down)  => if self.dir == Direction::Up    { self.dir } else { Direction::Down  },
            Some(Direction::Left)  => if self.dir == Direction::Right { self.dir } else { Direction::Left  },
            Some(Direction::Right) => if self.dir == Direction::Left  { self.dir } else { Direction::Right },
            None => self.dir,
        };

        let head = self.body[0];

        // add a new element in dir
        self.body.push_front(
            match self.dir {
                Direction::Up    => Pos { x: head.x, y: head.y + 1},
                Direction::Down  => Pos { x: head.x, y: head.y - 1},
                Direction::Left  => Pos { x: head.x - 1, y: head.y},
                Direction::Right => Pos { x: head.x + 1, y: head.y},
            });

        // check for food
        if self.world.food_pos == head {
            self.world.place_food(self.body.make_contiguous());
            self.grow_buf = 3;
        }

        if self.grow_buf > 0 {
            self.grow_buf -= 1;
        } else {
            // remove the tail of the snek
            self.body.pop_back();
        }
    }

    // TODO seg faults when resizing window 
    fn draw(&mut self, pixels: &mut Pixels) {
        // I don't know of a better way to do this.
        self.world.draw(pixels);

        //  1 pixel separation from each part.
        let body_color: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
        let head_color: [u8; 4] = [0xf7, 0xe8, 0x72, 0xff];

        let (win_width, win_height) = pixels.get_size();

        for (i, b) in self.body.iter().enumerate() {
            // map the position to pixel position. 0 is the center of the screen.
            let b = Pos::<u32> {
                x: ((b.x*self.world.block_size as i32) + (win_width/2)  as i32) as u32,
                y: ((-b.y*self.world.block_size as i32) + (win_height/2) as i32) as u32,
            };
            
            if !(0..win_width).contains(&b.x) | !(0..win_height).contains(&b.y){
                continue
            }

            let color: &[u8];
            if i == 0 {
                color = &head_color;
            } else {
                color = &body_color;
            }
            
            for y in 0..self.world.block_size-3 {
                for x in 0..self.world.block_size-3{
                    // calculate index in buffer, times four because of RGBA format.
                    pixels
                        .access_pixel((b.x+x) as usize, (b.y-y) as usize)
                        .expect("frame buffer issue[1]")
                        .copy_from_slice(color);
                }
            }
        }
    }
}

fn print_text(pixels: &mut Pixels, message: String, scale: rusttype::Scale, font: &rusttype::Font, start_pos: Pos<usize>) {
    let v_metrics = font.v_metrics(scale);
    let pos = rusttype::point(0.0, v_metrics.ascent);

    // pre-rasterize text
    let glyphs: Vec<_> = font
        .layout(&message, scale, pos)
        .collect();

    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let mut text_buf = vec![0; ((glyphs_width+20)*(glyphs_height)) as usize];

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let x = x as i32 + bounding_box.min.x;
                let y = y as i32 + bounding_box.min.y;

                let index = ((y as u32)*glyphs_width + x as u32) as usize;
                text_buf[index] = (255.0*v) as u8;
            })
        }
    }

    for x in 0..glyphs_width as usize {
        for y in 0..glyphs_height as usize {
            pixels
                .access_pixel(x + start_pos.x ,y + start_pos.y)
                .and_then::<Option<&[u8]>, _>(|f| {
                    let v = text_buf[(y*(glyphs_width as usize) + x)];
                    f[0] = v;
                    f[1] = v;
                    f[2] = v;
                    f[3] = 0;
                    None
                });
        }
    }
}

fn main() -> Result<(), Error> {
    const GAME_WIDTH: u32  = 40;
    const GAME_HEIGHT: u32 = 40;
    const BLOCK_SIZE: u32  = 20;
    // set up the window
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("ethans snake")
        .with_min_inner_size(dpi::PhysicalSize {
            width: BLOCK_SIZE*GAME_WIDTH + 30,
            height: BLOCK_SIZE*GAME_HEIGHT + 40,
        })
        .build(&event_loop).unwrap();

    // load fonts
    let font_bytes = include_bytes!("../fonts/VCR_OSD_MONO.ttf") as &[u8];
    let font = rusttype::Font::try_from_bytes(font_bytes).expect("font-loading error");


    // set up the renderer.
    let mut pixel_buffer = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)? 
    };
    

    let world = World::new(GAME_WIDTH, GAME_HEIGHT, BLOCK_SIZE);
    let mut sssss = SlitherySnek::new(3, world);
    
    let mut last_time = time::Instant::now();
    let mut move_timer = time::Duration::new(0,0);

    let mut is_dead = false;
    // if you try and do a fast move, you might accidentally overwrite the direction
    //  you originally wanted to go. this prevents that from happening.
    let mut input_q: VecDeque<Option<Direction>> = VecDeque::new();

    // Do event handling, window updating, etc...
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        let delta_time = last_time.elapsed();
        last_time = time::Instant::now();

        move_timer += delta_time;

        match event {
            // close the window if applicable
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            // window resizing
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                pixel_buffer.resize_surface(new_size.width, new_size.height);
                pixel_buffer.resize_buffer(new_size.width, new_size.height);
            },

            // handle input events
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { 
                    input,
                    ..},
                ..
            } => {
                if input.state == ElementState::Pressed {
                    let direction =  match input.virtual_keycode {
                        Some(VirtualKeyCode::Up)    | Some(VirtualKeyCode::W) => Some(Direction::Up),
                        Some(VirtualKeyCode::Down)  | Some(VirtualKeyCode::S) => Some(Direction::Down),
                        Some(VirtualKeyCode::Left)  | Some(VirtualKeyCode::A) => Some(Direction::Left),
                        Some(VirtualKeyCode::Right) | Some(VirtualKeyCode::D) => Some(Direction::Right),
                        _ => None,
                    };

                    if (input_q.len() < 2) & (!input_q.contains(&direction)) {
                        input_q.push_back(direction);
                    } else {
                        input_q.clear();
                        input_q.push_back(direction);
                    }
                }
            },

            Event::MainEventsCleared => {
                if is_dead {
                    if !input_q.is_empty() {
                        sssss.reset();
                        is_dead = false;
                    }
                } else {
                    if move_timer >= time::Duration::from_millis(125) {
                        move_timer = time::Duration::new(0,0);
                        sssss.slither(input_q.pop_front().and_then(|d| d));
                        is_dead = sssss.check_dead();
                        window.request_redraw();
                    }
                }

            },

            Event::RedrawRequested(_) => {
                pixel_buffer
                    .get_frame()
                    .fill(0);
                // Redraw the application
                sssss.draw(&mut pixel_buffer);

                // print score
                let score = format!("score: {}", &(sssss.body.len() as u32) + &sssss.grow_buf);
                print_text(&mut pixel_buffer, score, rusttype::Scale::uniform(20.0), &font, Pos { x: 0, y: 0});

                if is_dead == true {
                    let (x,y) = pixel_buffer.get_size();
                    let (x,y) = (x as usize, y as usize);

                    print_text(&mut pixel_buffer, "You died! press any key to try again...".to_string(), rusttype::Scale::uniform(30.0), &font, Pos { x: x/8, y: y/2});
                }

                if let Err(_) = pixel_buffer.render() {
                    return ();
                }
            },

            _ => (),
        }
    })

}
