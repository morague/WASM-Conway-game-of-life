mod game;
use game::Organism; 
use rand::Rng;
use js_sys::Function;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::{prelude::*, JsCast, UnwrapThrowExt};
use web_sys::{window, HtmlDivElement, HtmlElement};



const SPEED: u64 = 700;
// const SPEED_SEC: f32 =  (SPEED / 100) as f32;


thread_local! {
    static GAME: Rc<RefCell<Organism>> =
    Rc::new(RefCell::new(Organism::new()));

    static EPOCH: Closure<dyn FnMut()> =  Closure::wrap(Box::new(move || { 
        GAME.with(|game| game.borrow_mut().on_epoch());
        render(); 
    }) as Box<dyn FnMut()>);
}


#[wasm_bindgen]
pub fn main(width:usize, height:usize) {
    init_grid(width, height);
    GAME.with(|game| game.borrow_mut().genesis(width.try_into().unwrap(), 
        height.try_into().unwrap(), 
        rand::thread_rng().gen_range(1..101)));

    EPOCH.with(|tick_closure| {
        window()
          .unwrap_throw()
          .set_interval_with_callback_and_timeout_and_arguments_0(
            tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
            SPEED.try_into().unwrap(),
          )
          .unwrap_throw()
      });
    render();
}


pub type Position = (usize, usize);

pub fn init_grid(width: usize, height: usize) {
    let document = window().unwrap_throw().document().unwrap_throw();
    let game_grid = document
        .get_element_by_id("game")
        .unwrap_throw()
        .dyn_into::<HtmlElement>()
        .unwrap_throw();

    game_grid
        .style()
        .set_property("grid-template",
            &format!("repeat({}, auto) / repeat({}, auto)", height, width),
        )
        .unwrap_throw();

    let mut cell_index: usize = 0;

    for y in 0..height {
        for x in 0..width {
            let position: Position = (x + 1, y + 1);

            let render_cell = document
                .create_element("div")
                .unwrap_throw()
                .dyn_into::<HtmlDivElement>()
                .unwrap_throw();
    
            render_cell.set_class_name("cell");
            render_cell.set_id(&cell_index.to_string());
        
            render_cell
                .style()
                .set_property("grid-column", &(position.0).to_string())
                .unwrap_throw();
        
            render_cell
                .style()
                .set_property("grid-row", &(position.1).to_string())
                .unwrap_throw();
        
            render_cell
                .style()
                .set_property("border", "solid 1px white")
                .unwrap_throw();
        
            render_cell
                .style()
                .set_property("border-radius", "5px")
                .unwrap_throw();
    
            render_cell
                .style()
                .set_property("background-color", "white")
                .unwrap_throw();

            cell_index += 1;
            game_grid.append_child(&render_cell).unwrap_throw();
        }
    }
}



pub fn render() {
    GAME.with(|game| {
        let game = game.borrow();
        let document = window().unwrap_throw().document().unwrap_throw();

        for index in 0..(game.grid).len() {
            let cell = document
                .get_element_by_id(&(index).to_string())
                .unwrap_throw()
                .dyn_into::<HtmlElement>()
                .unwrap_throw();
    
                match game.grid[index].active {
                    true => set_as_active(cell, game.grid[index].was_active),
                    false => set_as_inactive(cell, game.grid[index].was_active)
                }
        } 
    })
}

pub fn set_as_active(cell: HtmlElement, pr_act: bool) {
    cell
        .style()
        .set_property("border", "solid 1px white")
        .unwrap_throw();

    cell
        .style()
        .set_property("border-radius", "5px")
        .unwrap_throw();

    if pr_act {
        cell
            .style()
            .set_property("background-color", "black")
            .unwrap_throw();

        cell
            .style()
            .set_property("animation", "appear 0s")
            .unwrap_throw();

    } else {
        cell
            .style()
            .set_property("background-color", "black")
            .unwrap_throw();

        cell
            .style()
            .set_property("animation", "appear 0.7s")
            .unwrap_throw();
    }
}

pub fn set_as_inactive(cell: HtmlElement, pr_act: bool) {
    
    cell
        .style()
        .set_property("border", "solid 1px white")
        .unwrap_throw();

    cell
        .style()
        .set_property("border-radius", "5px")
        .unwrap_throw();

    if pr_act {
        cell
            .style()
            .set_property("background-color", "black")
            .unwrap_throw();

        cell
            .style()
            .set_property("animation", "fade 0.7s")
            .unwrap_throw();
    
     } else {
        cell
            .style()
            .set_property("background-color", "white")
            .unwrap_throw();

        cell
            .style()
            .set_property("animation", "fade 0s")
            .unwrap_throw();
    }
}