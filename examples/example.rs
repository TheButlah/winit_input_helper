use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let _window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        // Pass every event to the WindowInputHelper.
        // It will return true when the last event has been processed and it is time to run your application logic.
        if input.update(&event) {
            // query keypresses this update
            if input.key_pressed_os(VirtualKeyCode::A) {
                println!("The 'A' key was pressed on the keyboard (OS repeating)");
            }

            if input.key_pressed(VirtualKeyCode::A) {
                println!("The 'A' key was pressed on the keyboard");
            }

            if input.key_released(VirtualKeyCode::Q) || input.close_requested() || input.destroyed()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed_scancode(17) {
                // 17 is W for QWERTY
                println!("The 'W' key was pressed on the keyboard (scan code)");
            }

            if input.key_pressed_os_scancode(18) {
                // 18 is E for QWERTY
                println!("The 'E' key was pressed on the keyboard (scan code, Os Repeating)");
            }

            if input.key_held_scancode(19) {
                // 19 is R for QWERTY
                println!("The 'R' key is held (scan code)");
            }

            // query the change in cursor this update
            let cursor_diff = input.cursor_diff();
            if cursor_diff != (0.0, 0.0) {
                println!("The cursor diff is: {:?}", cursor_diff);
                println!("The cursor position is: {:?}", input.cursor());
            }

            // query the change in mouse this update (useful for camera)
            let mouse_diff = input.mouse_diff();
            if mouse_diff != (0.0, 0.0) {
                println!("The mouse diff is: {:?}", mouse_diff);
            }

            let scroll_diff = input.scroll_diff();
            if scroll_diff != (0.0, 0.0) {
                println!("The scroll diff is: {:?}", scroll_diff);
            }

            // You are expected to control your own timing within this block.
            // Usually via rendering with vsync.
            // render();
        }
    });
}
