#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate winit;
extern crate find_folder;
#[macro_use] extern crate conrod;


use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use winit::{Event, EventsLoop, Window, WindowEvent, ControlFlow};
use conrod::backend::glium::glium::{self, Surface};
use conrod::{widget, Positionable, Colorable, Widget};

//Rectangle aber da fehlen save noch importe
use conrod::{Color, Dimensions, Point, Rect, Sizeable};


fn write() -> std::io::Result<()> {
    let mut file = File::create("bu.txt")?;
    file.write_all(b"jajajajajjaja")?;
    Ok(())
}

fn read() -> std::io::Result<()> {
    let mut file = File::open("bu.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
     println!("With text:\n{}", contents);
    Ok(())
}

fn reading() -> std::string::String{
   let mut f = File::open("bu.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
   // println!("With text:\n{}", contents);
    return contents;

}



#[get("/")]
fn index() -> &'static str {
      "Hier soll auch das von der Txt Datei rein"
}

fn scan() -> std::string::String{

  let mut input = String::new();

    println!("eingabe");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Geschrieben {}", input);
        }
        Err(_e) => println!("error")
    }
return input;
}

fn winit(){
	let mut events_loop = winit::EventsLoop::new();
	let window = winit::Window::new(&events_loop).unwrap();

	events_loop.run_forever(|event| {
		match event {
			winit::Event::WindowEvent {
				event: winit::WindowEvent::CloseRequested,
				..			
			} => winit::ControlFlow::Break,	
			_ => winit::ControlFlow::Continue,	
		}	
	});
}


fn winitcustom(){
	let mut events_loop = winit::EventsLoop::new();
	let window = winit::Window::new(&events_loop).unwrap();

	events_loop.run_forever(|event| {
		match event {
			winit::Event::WindowEvent {event: winit::WindowEvent::CloseRequested, ..
			}
				 => {
				println!("Fenster wurde geschlossen");
				winit::ControlFlow::Break
		         	     },	
						_ => winit::ControlFlow::Continue,	
					}	
	});
}

/*#[derive(Clone, WidgetCommon_)]
pub struct Button<'a,S> {
	common: widget::CommonBuilder,
	maybe_label: Option<&'a str>,
	pub show: S,
	pub style: Style,
	enabled: bool,
}*/

fn conrodwindow(){
const WIDTH: u32 = 400;
const HEIGHT: u32 = 200;

let mut events_loop = glium::glutin::EventsLoop::new();
let window = glium::glutin::WindowBuilder::new()
                .with_title("Hello Conrod")
                .with_dimensions(WIDTH, HEIGHT);
let context = glium::glutin::ContextBuilder::new()
                .with_vsync(true)
                .with_multisampling(4);
let display = glium::Display::new(window, context, &events_loop).unwrap();

let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

'main: loop {
    // Render the `Ui` and then display it on the screen.
    if let Some(primitives) = ui.draw_if_changed() {
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 0.0, 1.0);
        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
    }
 }
}

fn fninfnaufrufen(){
	println!("Hier {}", reading());
}
	
fn conrodaustxtdatei(){


	const WIDTH: u32 = 400;
	const HEIGHT: u32 = 500;

	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
       	         .with_title("Rust UML Tool")
       	         .with_dimensions(WIDTH, HEIGHT);
	let context = glium::glutin::ContextBuilder::new()
       	         .with_vsync(true)
       	         .with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();

	let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

	widget_ids!(struct Ids { Schrift });
	let ids = Ids::new(ui.widget_id_generator());

	let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

	let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

	let assets = find_folder::Search::KidsThenParents(3, 5)
    		.for_folder("assets")
    		.unwrap();
	let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
	ui.fonts.insert_from_file(font_path).unwrap();



'main: loop {
    // Render the `Ui` and then display it on the screen.
    if let Some(primitives) = ui.draw_if_changed() 
{
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);

	let ui = &mut ui.set_widgets();

//Funktionsaufruf bei String übergabe mit &
//Siehe widget::Text::new(&reading())
	widget::Text::new(&reading())
		.middle_of(ui.window)
		.color(conrod::color::WHITE)
		.font_size(32)
		.set(ids.Schrift, ui);

        	renderer.draw(&display, &mut target, &image_map).unwrap();
        	target.finish().unwrap();
    }
 }
}

fn conrodcustom(){


	const WIDTH: u32 = 400;
	const HEIGHT: u32 = 500;

	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
        	        .with_title("Rust UML Tool")
        	        .with_dimensions(WIDTH, HEIGHT);
	let context = glium::glutin::ContextBuilder::new()
        	        .with_vsync(true)
        	        .with_multisampling(4);
	let display = glium::Display::new(window, context, &events_loop).unwrap();

	let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

//Widget wird erstellt
	widget_ids!(struct Ids { Schrift });
	let ids = Ids::new(ui.widget_id_generator());

//Rectangle
	pub struct Rectangle {

}

	let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

	let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

//Das ist für die Schriftart
	let assets = find_folder::Search::KidsThenParents(3, 5)
 	   .for_folder("assets")
 	   .unwrap();
	let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
	ui.fonts.insert_from_file(font_path).unwrap();



'main: loop {
    // Render the `Ui` and then display it on the screen.
    if let Some(primitives) = ui.draw_if_changed() 
{
        renderer.fill(&display, primitives, &image_map);
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);

//Un das Widget in der ui zu setzten
	let ui = &mut ui.set_widgets();

//Funktionsaufruf bei String übergabe mit &
//Siehe widget::Text::new(&reading())
	widget::Text::new(&reading())
		.middle_of(ui.window)
		.color(conrod::color::WHITE)
		.font_size(32)
		.set(ids.Schrift, ui);

        renderer.draw(&display, &mut target, &image_map).unwrap();
        target.finish().unwrap();
    }
 }
}

fn main() {
   // let emm = reading();
   // println!("hier {}", emm);
   // write();
   // reading();
   // println!("{}", reading());
   // println!("hier\n {}", te);
   // println!("hier {}",contents);
   // rocket::ignite().mount("/", routes![index]).launch();
   // scan();
   // println!("Das ist eingegeben worden{}",scan());
   // winit();
   // fninfnaufrufen();
   // conrodwindow();
    conrodaustxtdatei();

}
