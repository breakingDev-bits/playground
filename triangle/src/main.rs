// Custom modules
mod draw_in_terminal;
mod objects;

use draw_in_terminal::draw_in_terminal;
use objects::*;

mod winsdl;
use winsdl::{Winsdl};


// System modules
use sdl2::event::Event;
use std::thread;
use std::time::Duration;
use std::{f32::consts::PI, time::Instant};

use crate::objects::create_program;

fn start() {
    draw_in_terminal();
    thread::sleep(Duration::from_millis(1000));
    println!("А теперь также в окошке");
}
// Я делал по туториалу. Почти не понимаю задумку, но понимаю код. В будущем напишу что-то подобное
fn main() -> Result<(), String> {
    start();

    let mut state = Winsdl::new(1000, 1000)?;
   unsafe { gl::Viewport(0, 0, 1000, 1000); }

    let program = create_program().unwrap();
    program.set();

    let (mut vertices, mut indices) = triangle_fan(5);

    let vbo = Vbo::r#gen();
    vbo.set(&vertices);

    let vao = Vao::r#gen();
    vao.set();

    let ibo = Ibo::r#gen();
    ibo.set(&indices);

    let start: Instant = Instant::now();
    let mut seconds_elapsed: u32 = 0;

    'running: loop {
        for event in state.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {  }
            }
        }
        unsafe {
            gl::ClearColor(54./255., 159./255., 219./255., 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            if start.elapsed().as_secs_f32().floor() as u32 > seconds_elapsed {
                seconds_elapsed += 1;
                
                (vertices, indices) = triangle_fan(seconds_elapsed + 3);
                vbo.set(&vertices);
                ibo.set(&indices);
            }

            gl::DrawElements(
                gl::TRIANGLES, 
                indices.len() as i32, 
                gl::UNSIGNED_INT, 
                0 as *const _
            );
        }
        state.window.gl_swap_window();
    }

    Ok(())
}

fn triangle_fan(n: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = vec![
        0.0, 0.0,
        0.5, 0.0,
    ];
    let mut indices: Vec<u32> = vec![];

    let mut angle: f32;
    for m in 1..n {
        angle = 2. * PI * m as f32 / n as f32;

        vertices.push(angle.cos() * 0.5);
        vertices.push(angle.sin() * 0.5);

        indices.push(0);
        indices.push(m);
        indices.push(m+1);
    }

    indices.push(0);
    indices.push(n);
    indices.push(1);

    (vertices, indices)
}