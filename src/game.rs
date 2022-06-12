use std::vec::Vec;
use std::cmp::Ordering;
use std::slice::Iter;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rand::Rng;
use num_traits::FromPrimitive;

use self::Neighbourhood::*;





#[derive(Clone, Hash)]
pub struct Organism {
  epoch: usize,  
  width: isize,
  height: isize,
  pub grid: Vec<Cell>,
}


impl Organism {
  pub fn new() -> Self {
      Self {
          epoch: 0,
          width: 0,
          height: 0,
          grid: Vec::new(),
      }
  }


  pub fn builder(&mut self, width: isize, height: isize) {
      self.width = width;
      self.height = height;

      for n in 0..self.height {
          for m in 0..self.width {
              let cell = Cell::new(m, n, false, 0);
              self.grid.push(cell);
          }
      }
  }


  pub fn rng_genesis(&mut self, value: &u8) {
      /*Turn randomly cells to active status*/
      for i in 0..self.grid.len() {
          let treshold: u8 = rand::thread_rng().gen_range(1..101);
          let _ = &mut self.grid[i].live_origins(&treshold, &value);                   
      }
  }

  pub fn check_neighbourhood(&mut self) {
      // position 0 to width, 0 to max row
      for i in 0..self.grid.len() {

          for direction in Neighbourhood::iterator() {
              let index = self.grid[i].get_neighbours_index(self.width, self.height, direction);

              if index != None {
                  let cell_index: usize =  FromPrimitive::from_isize(index.unwrap()).unwrap();
                  let cell: Cell = self.grid[cell_index].clone();
                  let _ = &mut self.grid[i].count_as_active_neighbour(&cell); 
              }
          }
      }
  }

  
  pub fn evoluate(&mut self) {
      for i in 0..self.grid.len() {
          let _ = &mut self.grid[i].apply_conway_rules();
          
      }
  }


  pub fn organism_state(&self) -> usize {

      let mut life_state: usize = 0;

      for i in 0..self.grid.len() {
          if self.grid[i].active {
              life_state += 1;
          }
      }
      life_state
  }


  pub fn genesis(&mut self, width: isize, height: isize, life_degree: u8){
    self.builder(width, height);
    self.rng_genesis(&life_degree);
  }

  pub fn on_epoch(&mut self) {

    let hash_start = self.get_hash();

    self.epoch += 1;
    self.check_neighbourhood();
    self.evoluate();
    let life_state = self.organism_state();
    let life_degree = rand::thread_rng().gen_range(1..101);

    let hash_end = self.get_hash();


    if life_state < 50 {
        self.rng_genesis(&life_degree);
        self.epoch = 0;

    } else if hash_start == hash_end {
        self.rng_genesis(&life_degree);
        self.epoch = 0; 

    } else if self.epoch == 1000 {
        self.rng_genesis(&life_degree);
        self.epoch = 0;
    }
  }

  fn get_hash(&self) -> u64 {
    let mut hasher = DefaultHasher::new();
    self.grid.hash(&mut hasher);
    hasher.finish()
  }
}










pub enum  Neighbourhood {
  TopLeft,
  Top,
  TopRight,
  Left,
  Right,
  BottomLeft,
  Bottom,
  BottomRight,
}

impl Neighbourhood {
  pub fn iterator() -> Iter<'static, Neighbourhood> {
      static DIRECTIONS: [Neighbourhood; 8] = [TopLeft, Top, TopRight,Left,
                                              Right,BottomLeft,Bottom,BottomRight];
      DIRECTIONS.iter()
  }
}






pub type Position = (isize, isize);

#[derive(Clone, Hash)]
pub struct Cell {
  position: Position,
  pub active: bool,
  pub was_active: bool,
  neighbours: u8,
}

impl Cell {
  pub fn new(x:isize, y:isize, active:bool, neighbours:u8) -> Self {
      Cell {
          position: (x, y),
          active: active,
          was_active: false,
          neighbours: neighbours,
      }
  }

  pub fn live_origins(&mut self, &treshold: &u8, &value: &u8) {
      match treshold.cmp(&value) {
          Ordering::Less => self.active = true,
          Ordering::Equal => self.active = true,
          Ordering::Greater => self.active = false,
      }
  }


  fn get_neighbours_index(&self, width: isize, height: isize, direction: &Neighbourhood) -> Option<isize> {
      // self.position is the center
      match &direction {
          Neighbourhood::TopLeft => self.get_index(width, height, (-1, -1)),
          Neighbourhood::Top => self.get_index(width, height, (0, -1)),
          Neighbourhood::TopRight => self.get_index(width, height, (1, -1)),
          Neighbourhood::Left => self.get_index(width, height, (-1, 0)),
          Neighbourhood::Right => self.get_index(width, height, (1, 0)),
          Neighbourhood::BottomLeft => self.get_index(width, height, (-1, 1)),
          Neighbourhood::Bottom => self.get_index(width, height, (0, 1)),
          Neighbourhood::BottomRight => self.get_index(width, height, (1, 1)),
      }
  }

  fn get_index(&self, width: isize, height: isize, direction: Position) -> Option<isize> {

      let end_col_token: isize = height - 1;
      let end_row_token: isize = width - 1;


      let basis: isize = &self.position.0 + &self.position.1 * width;
      let distance: isize = direction.0 + direction.1 * width;
      let index: isize = basis + distance;

      if direction == (-1, -1) {
          if index < 0 || self.position.0 == 0 {
              None
          } else {
              Some(index)
          }
      
      }  else if direction == (0, -1) {
          // top nodes
          if index < 0  {
              None
          } else {
              Some(index)
          }

      } else if direction == (1, -1) {
          if index < 0 || self.position.0 == end_row_token {
              None
          } else {
              Some(index)         
          }

      } else if direction == (-1, 0) {
          if self.position.0 == 0 {
              None
          } else {
              Some(index)
          } 

      } else if direction == (1, 0) {
          if self.position.0 == end_row_token {
              None
          } else {
              Some(index)
          }

      } else if direction == (-1, 1) {
          if self.position.0 == 0 || self.position.1 == end_col_token {
              None
          } else {
              Some(index)
          }

      } else if direction == (0, 1) {
          if self.position.1 == end_col_token {
              None
          } else {
              Some(index)
          }

      } else {
          if self.position.0 == end_row_token || self.position.1 == end_col_token {
              None
          } else {
              Some(index)
          }
      }
  }

  pub fn count_as_active_neighbour(&mut self, cell: &Cell) {
      if cell.active {
          self.neighbours += 1;
      }
  }



  //RULES

  pub fn apply_conway_rules(&mut self) {
      if (self.neighbours  == 2 || self.neighbours == 3)
          && self.active {
          self.was_active = self.active;
          self.active = true;

      } else if self.neighbours == 3 && self.active== false {
          self.was_active = self.active;
          self.active = true;

      } else {
          self.was_active = self.active;
          self.active = false;
      }
      self.neighbours = 0;
  }
}