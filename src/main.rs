extern crate gl;
extern crate sdl2;

fn main() {
    /* SDL init */
    let _sdl = sdl2::init().unwrap();
    /* Run Video Subsystem */
    let video_subsystem = _sdl.video().unwrap();
    let _window = video_subsystem
        .window("Zenith EDDI Lidar Map", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = _window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    /* Receive Video Window Event */
    let mut event_pump = _sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        _window.gl_swap_window();
    }
}
