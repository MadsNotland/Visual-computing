// Uncomment these following global attributes to silence most warnings of "low" interest:
/*
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_variables)]
*/
extern crate nalgebra_glm as glm;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::{mem, os::raw::c_void, ptr};

mod mesh;
mod scene_graph;
mod shader;
mod toolbox;
mod util;

use glutin::event::{
    DeviceEvent,
    ElementState::{Pressed, Released},
    Event, KeyboardInput,
    VirtualKeyCode::{self, *},
    WindowEvent,
};
use glutin::event_loop::ControlFlow;
use scene_graph::SceneNode;

// initial window size
const INITIAL_SCREEN_W: u32 = 800;
const INITIAL_SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //

// Get the size of an arbitrary array of numbers measured in bytes
// Example usage:  byte_size_of_array(my_array)
//Brukes for å finne ut hvor mange bytes et array tar opp i minnet, for å kunne sende det til GPUen
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
// Example usage:  pointer_to_array(my_array)
// Forteller hvor i minnet arrayet starter, og kan brukes til å sende data til GPUen
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
// Example usage:  size_of::<u64>()
// Brukes for å finne ut hvor mange bytes en type tar opp i minnet, for å kunne sende det til GPUen
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T, represented as a relative pointer
// Example usage:  offset::<u64>(4)
// Brukes for å finne ut hvor mange bytes et array tar opp i minnet, for å kunne sende det til GPUen
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()
//Oppgave:
//Sett opp en funksjon som setter opp en Vertex Array Object (VAO) som inneholder flere triangler
//Funksjonen skal ta inn en vektor av tre dimensjononale vertex koordinater og en array av indekser
//Kan anta at innholdet i bufferen utelukkende består av trekanter, spesifisert av flyttalskoordinater

// == // Generate your VAO here
unsafe fn create_vao(
    vertices: &Vec<f32>,
    colors: &Vec<f32>,
    normals: &Vec<f32>,
    indices: &Vec<u32>,
) -> u32 {
    // Implement me!

    // This should:
    // * Generate a VAO and bind it, VAO lagrer konfigurasjonen for vertex-attributtene og hvilken IBO/EBO som er bundet    let mut vao: u32 = 0; //Lager en variabel som holder ID til vao. Trenger mut fordi OpenGL skal endre verdien fra 0 til en faktisk ID
    let mut vao: u32 = 0;
    gl::GenVertexArrays(1, &mut vao); // Genererer et VAO, og lagrer IDen i vao
    gl::BindVertexArray(vao); //Gjør VAO aktivt, slik at vi kan konfigurere det

    // * Generate a VBO and bind it, lagrer dataene i GPU minnet, og gjør det tilgjengelig for OpenGL
    let mut vbo: u32 = 0;
    gl::GenBuffers(1, &mut vbo); //Genererer 1 buffer, og lagrer IDen i vbo
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo); //Gjør VBO aktivt, slik at vi kan fylle det med data

    // * Fill it with data
    gl::BufferData(
        gl::ARRAY_BUFFER,             //Type av buffer
        byte_size_of_array(vertices), //Størrelse på dataene i bytes
        pointer_to_array(vertices),   //Peker til dataene
        gl::STATIC_DRAW, //Hvordan dataene skal brukes, STATIC_DRAW betyr at dataene ikke vil endres ofte
    );

    // * Configure a VAP for the data and enable it, dette forteller OpenGL hvordan dataene i VBOen skal tolkes, og hvilke attributter som skal brukes
    gl::VertexAttribPointer(
        0,                    //index til attributtet, 0 betyr at det er det første attributtet
        3,         //antall komponenter i dette attributtet, 3 betyr at det er 3D koordinater
        gl::FLOAT, //type av dataene, FLOAT betyr at det er flyttall
        gl::FALSE, //om dataene skal normaliseres, FALSE betyr at de ikke skal normaliseres
        size_of::<f32>() * 3, //stride, hvor mange bytes mellom hver vertex, 3 * size_of::<f32>() betyr at det er 3 flyttall per vertex
        offset::<f32>(0),     //offset, hvor mange bytes fra starten av VBO
    );

    gl::EnableVertexAttribArray(0); //Aktiverer det første attributtet på lokasjon 0

    let mut color_vbo: u32 = 0;
    gl::GenBuffers(1, &mut color_vbo); //Genererer 1 buffer, og lagrer IDen i color_vbo
    gl::BindBuffer(gl::ARRAY_BUFFER, color_vbo); //Gjør VBO aktivt, slik at vi kan fylle det med data

    gl::BufferData(
        gl::ARRAY_BUFFER,           //Type av buffer
        byte_size_of_array(colors), //Størrelse på dataene i bytes
        pointer_to_array(colors),   //Peker til dataene
        gl::STATIC_DRAW, //Hvordan dataene skal brukes, STATIC_DRAW betyr at dataene ikke vil endres ofte
    );
    gl::VertexAttribPointer(
        1,
        4,
        gl::FLOAT,
        gl::FALSE,
        size_of::<f32>() * 4,
        offset::<f32>(0),
    );
    gl::EnableVertexAttribArray(1);

    let mut normal_vbo: u32 = 0;
    gl::GenBuffers(1, &mut normal_vbo); //Genererer 1
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_vbo); //Gjør VBO aktivt, slik at vi kan fylle det med data
    gl::BufferData(
        gl::ARRAY_BUFFER,
        byte_size_of_array(normals),
        pointer_to_array(normals),
        gl::STATIC_DRAW,
    );
    gl::VertexAttribPointer(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        size_of::<f32>() * 3,
        offset::<f32>(0),
    );
    gl::EnableVertexAttribArray(2);

    let mut ibo: u32 = 0;
    gl::GenBuffers(1, &mut ibo); //Genererer 1 buffer, og lagrer IDen i ibo
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo); //Gjør IBO aktivt, slik at vi kan fylle det med data
                                                   // * Fill it with data
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        byte_size_of_array(indices),
        pointer_to_array(indices),
        gl::STATIC_DRAW,
    );
    // * Return the ID of the VAO
    vao //Returnerer IDen til VAO, som kan brukes senere for å binde det og tegne det
}
unsafe fn draw_scene(
    node: &scene_graph::SceneNode,
    view_projection_matrix: &glm::Mat4,
    transformation_so_far: &glm::Mat4,
    transformation_location: i32,
    model_matrix_location: i32,
) {
    let translation = glm::translation(&node.position);
    let to_origin = glm::translation(&(-node.reference_point));
    let from_origin = glm::translation(&node.reference_point);

    let rotation_x = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
    let rotation_y = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
    let rotation_z = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));

    //matrise fra høyremot venstre
    let relative_transformation =
        translation * from_origin * rotation_x * rotation_y * rotation_z * to_origin;

    let current_transformation = transformation_so_far * relative_transformation;
    let mvp = view_projection_matrix * current_transformation; //MVP = Projection * View * Model

    gl::UniformMatrix4fv(transformation_location, 1, gl::FALSE, mvp.as_ptr());
    gl::UniformMatrix4fv(
        model_matrix_location,
        1,
        gl::FALSE,
        current_transformation.as_ptr(),
    );
    if node.index_count > 0 {
        gl::BindVertexArray(node.vao_id);

        gl::DrawElements(
            gl::TRIANGLES,
            node.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }

    for &child in &node.children {
        draw_scene(
            &*child,
            view_projection_matrix,
            &current_transformation,
            transformation_location,
            model_matrix_location,
        );
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(true)
        .with_inner_size(glutin::dpi::LogicalSize::new(
            INITIAL_SCREEN_W,
            INITIAL_SCREEN_H,
        ));
    let cb = glutin::ContextBuilder::new().with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Set up shared tuple for tracking changes to the window size
    let arc_window_size = Arc::new(Mutex::new((INITIAL_SCREEN_W, INITIAL_SCREEN_H, false)));
    // Make a reference of this tuple to send to the render thread
    let window_size = Arc::clone(&arc_window_size);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers.
        // This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        let mut window_aspect_ratio = INITIAL_SCREEN_W as f32 / INITIAL_SCREEN_H as f32;

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!(
                "{}: {}",
                util::get_gl_string(gl::VENDOR),
                util::get_gl_string(gl::RENDERER)
            );
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!(
                "GLSL\t: {}",
                util::get_gl_string(gl::SHADING_LANGUAGE_VERSION)
            );
        }

        // == // Set up your shaders here

        let simple_shader = unsafe {
            //unsafe betyr at vi gjør operasjoner som kompilatoren ikke kan garantere er minnesikre.
            shader::ShaderBuilder::new()
                .attach_file("./shaders/simple.vert")
                .attach_file("./shaders/simple.frag")
                .link()
        };

        let model_matrix_location = unsafe {
            gl::GetUniformLocation(
                simple_shader.program_id,
                b"modelMatrix\0".as_ptr() as *const i8,
            )
        };
        let camera_position_location = unsafe {
    gl::GetUniformLocation(
        simple_shader.program_id,
        b"cameraPosition\0".as_ptr() as *const i8,
    )
};


        unsafe {
            simple_shader.activate();
        }

        let transformation_location = unsafe {
            let uniform_name = std::ffi::CString::new("transformation").unwrap();

            gl::GetUniformLocation(simple_shader.program_id, uniform_name.as_ptr())
        };

        let mut camera_x: f32 = -30.0;
        let mut camera_y: f32 = 20.0;
        let mut camera_z: f32 = -40.0;

        let mut camera_yaw: f32 = -0.6;
        let mut camera_pitch: f32 = 0.0;

        // The main rendering loop
        let first_frame_time = std::time::Instant::now();
        let mut previous_frame_time = first_frame_time;

        //laster inn terrenget
        let terrain = mesh::Terrain::load("resources/lunarsurface.obj");

        let terrain_vao = unsafe {
            create_vao(
                &terrain.vertices,
                &terrain.colors,
                &terrain.normals,
                &terrain.indices,
            )
        };

        let helicopter = mesh::Helicopter::load("resources/helicopter.obj");

        let mut root = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain.index_count);
        let mut helicopter_roots: Vec<scene_graph::Node> = Vec::new();
        let body_vao = unsafe {
            create_vao(
                &helicopter.body.vertices,
                &helicopter.body.colors,
                &helicopter.body.normals,
                &helicopter.body.indices,
            )
        };
        let main_rotor_vao = unsafe {
            create_vao(
                &helicopter.main_rotor.vertices,
                &helicopter.main_rotor.colors,
                &helicopter.main_rotor.normals,
                &helicopter.main_rotor.indices,
            )
        };
        let door = unsafe {
            create_vao(
                &helicopter.door.vertices,
                &helicopter.door.colors,
                &helicopter.door.normals,
                &helicopter.door.indices,
            )
        };

        let tail = unsafe {
            create_vao(
                &helicopter.tail_rotor.vertices,
                &helicopter.tail_rotor.colors,
                &helicopter.tail_rotor.normals,
                &helicopter.tail_rotor.indices,
            )
        };

        for _ in 0..5 {
            let mut helicopter_root = SceneNode::new();

            helicopter_root.position.y = 5.0;
            let body_node = SceneNode::from_vao(body_vao, helicopter.body.index_count);

            let main_rotor_node =
                SceneNode::from_vao(main_rotor_vao, helicopter.main_rotor.index_count);

            let door_node = SceneNode::from_vao(door, helicopter.door.index_count);

            let mut tail_node = SceneNode::from_vao(tail, helicopter.tail_rotor.index_count);

            tail_node.reference_point = glm::vec3(0.35, 2.3, 10.4);

            helicopter_root.add_child(&body_node);
            helicopter_root.add_child(&main_rotor_node);
            helicopter_root.add_child(&door_node);
            helicopter_root.add_child(&tail_node);

            terrain_node.add_child(&helicopter_root);

            helicopter_roots.push(helicopter_root);
        }
        root.add_child(&terrain_node);

        loop {
            // Compute time passed since the previous frame and since the start of the program
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(previous_frame_time).as_secs_f32();
            previous_frame_time = now;

            for (i, helicopter_root) in helicopter_roots.iter_mut().enumerate() {
                let offset = i as f32 * 1.0;
                let heading = toolbox::simple_heading_animation(elapsed + offset);

                helicopter_root.position.x = heading.x;
                helicopter_root.position.z = heading.z;

                helicopter_root.rotation.x = heading.pitch;
                helicopter_root.rotation.y = heading.yaw;
                helicopter_root.rotation.z = heading.roll;

                helicopter_root[1].rotation.y = elapsed * 10.0;
                helicopter_root[3].rotation.x = elapsed * 10.0;
            }

            // Handle resize events
            if let Ok(mut new_size) = window_size.lock() {
                if new_size.2 {
                    context.resize(glutin::dpi::PhysicalSize::new(new_size.0, new_size.1));
                    window_aspect_ratio = new_size.0 as f32 / new_size.1 as f32;
                    (*new_size).2 = false;
                    println!("Window was resized to {}x{}", new_size.0, new_size.1);
                    unsafe {
                        gl::Viewport(0, 0, new_size.0 as i32, new_size.1 as i32);
                    }
                }
            }
            let mut movement = glm::vec3(0.0, 0.0, 0.0);
            // Handle keyboard input

            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::W => {
                            movement[2] -= 1.0;
                        }

                        VirtualKeyCode::S => {
                            movement[2] += 1.0;
                        }

                        VirtualKeyCode::A => {
                            movement[0] -= 1.0;
                        }

                        VirtualKeyCode::D => {
                            movement[0] += 1.0;
                        }
                    
                        VirtualKeyCode::Space => {
                            movement[1] += 2.0;
                        }

                        VirtualKeyCode::LShift => {
                            movement[1] -= 2.0;
                        }
                        VirtualKeyCode::Left => {
                            camera_yaw += delta_time;
                        }

                        VirtualKeyCode::Right => {
                            camera_yaw -= delta_time;
                        }
                        VirtualKeyCode::Up => {
                            camera_pitch += delta_time;
                        }

                        VirtualKeyCode::Down => {
                            camera_pitch -= delta_time;
                        }

                        VirtualKeyCode::O => {
                            for helicopter_root in helicopter_roots.iter_mut() {
                                helicopter_root[2].position.z = 2.0;
                            }
                        }
                        VirtualKeyCode::P => {
                            for helicopter_root in helicopter_roots.iter_mut() {
                                helicopter_root[2].position.z = 0.0;
                            }
                        }

                        _ => {}
                    }
                }
            }
            let camera_rotation: glm::Mat4 = glm::rotation(camera_yaw, &glm::vec3(0.0, 1.0, 0.0))
                * glm::rotation(camera_pitch, &glm::vec3(1.0, 0.0, 0.0));
            let world_movement =
                camera_rotation * glm::vec4(movement[0], movement[1], movement[2], 0.0);
            camera_x += world_movement[0] * delta_time;
            camera_y += world_movement[1] * delta_time;
            camera_z += world_movement[2] * delta_time;
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {
                // == // Optionally access the accumulated mouse movement between
                // == // frames here with `delta.0` and `delta.1`

                *delta = (0.0, 0.0); // reset when done
            }

            // == // Please compute camera transforms here (exercise 2 & 3)
            let translation: glm::Mat4 =
                glm::translation(&glm::vec3(-camera_x, -camera_y, -camera_z));
            let yaw_rotation: glm::Mat4 = glm::rotation(-camera_yaw, &glm::vec3(0.0, 1.0, 0.0));

            let pitch_rotation: glm::Mat4 = glm::rotation(-camera_pitch, &glm::vec3(1.0, 0.0, 0.0));
            let perspective: glm::Mat4 =
                glm::perspective(window_aspect_ratio, 45.0_f32.to_radians(), 1.0, 1000.0);
            let transformation: glm::Mat4 =
                perspective * pitch_rotation * yaw_rotation * translation;

            unsafe {
                gl::ClearColor(0.035, 0.046, 0.078, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                simple_shader.activate();

                gl::UniformMatrix4fv(
                    transformation_location,
                    1,
                    gl::FALSE,
                    transformation.as_ptr(),
                );
 gl::Uniform3f(
        camera_position_location,
        camera_x,
        camera_y,
        camera_z,
    );
                draw_scene(
                    &root,
                    &transformation,
                    &glm::identity(),
                    transformation_location,
                    model_matrix_location,
                );

            }
            // Display the new color buffer on the display
            context.swap_buffers().unwrap(); // we use "double buffering" to avoid artifacts
        }
    });

    // == //
    // == // From here on down there are only internals.
    // == //

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events are initially handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!(
                    "New window size received: {}x{}",
                    physical_size.width, physical_size.height
                );
                if let Ok(mut new_size) = arc_window_size.lock() {
                    *new_size = (physical_size.width, physical_size.height, true);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: key_state,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        }
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle Escape and Q keys separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            }
            _ => {}
        }
    });
}


