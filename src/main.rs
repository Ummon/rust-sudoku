use std::{io, os, char, result};

fn print_usage(exe_name: &str) {
   println!("Usage: {} <filename>", exe_name);
}

struct Board {
   data: [[u8, ..9], ..9]
}

#[deriving(Show)]
struct Pos {
   i: uint,
   j: uint
}

impl Board {
   fn new() -> Board {
      Board { data: [[0, ..9], ..9] }
   }
   
   #[inline(always)]
   fn get(&self, i: uint , j: uint) -> u8 {
      self.data[i][j]
   }
   
   #[inline(always)]
   fn get_pos(&self, pos: &Pos) -> u8 {
      self.data[pos.i][pos.j]
   }
  
   /* Unused.
   #[inline(always)]
   fn set(&mut self, v: u8, i: uint, j: uint) {
      self.data[i][j] = v;
   }*/
   
   #[inline(always)]
   fn set_pos(&mut self, v: u8, pos: &Pos) {
      self.data[pos.i][pos.j] = v;
   }
   
   fn write(&self, output: &mut io::Writer) -> Result<(), io::IoError> {   
      for i in range(0, self.data.len()) {
         for j in range(0, self.data[i].len()) {
            try!(output.write_uint(self.data[i][j] as uint));
            if (j + 1) % 3 == 0 && j != self.data[i].len() - 1 {
               try!(output.write_str("|"));
            }
         }
         
         try!(output.write_str("\n"));
         
         if (i + 1) % 3 == 0 && i != self.data.len() - 1 {
          
          try!(output.write_line("-----------"));
         }
      }
      
      result::Ok(())
   }

   fn read(input: &mut io::Reader) -> Result<Board, io::IoError> {
      let mut board = Board::new();   
      {
         let mut pos = Pos { i: 0, j: 0 }; // Current position
         
         let assign = |v: u8| -> bool {
            board.set_pos(v, &pos);
            if pos.i == 8 && pos.j == 8 {
               true
            } else {
               pos = Board::next(pos);
               false
            }
         };

         loop {
            match input.read_byte() {
               Ok(byte) => {
                  let c = byte as char;
                  if c == ' ' || c == '.' {
                     if assign(0u8) {
                        break;
                     }
                  } else {
                     match char::to_digit(byte as char, 10){
                        Some(v) => {
                           if assign(v as u8) {
                              break;
                           }
                        },
                        None => continue
                     };
                  }
               },
               Err(err) => 
                  return Err(err)
            }
         }
      }
      
      Ok(board)
   }
         
   /**
     * Returns the next position. The first position is returned if the given position is the last.
     */
   #[inline(always)]
   fn next(pos: Pos) -> Pos {
      let mut next_pos = pos;
      next_pos.j += 1;
      if next_pos.j >= 9 {
         next_pos.j = 0;
         next_pos.i += 1;
         if next_pos.i >= 9 {
            next_pos.i = 0;
         }
      };
      next_pos
   }
   
   /**
     * Returns the next free position (containing a 0).
     */
   #[inline(always)]
   fn next_free(&self, pos: Pos) -> Option<Pos> {
      let mut current = pos;
      
      loop {          
         if current.i == 8 && current.j == 8 {
            return None;
         }     
         
         current = Board::next(current); 
         
         if self.get_pos(&current) == 0 {
            return Some(current);
         }
      }
   }
   
   /**
     * Return 0 if there is some or no valid number else return the number.
     */
   fn the_only_valid_number(&self, pos: &Pos) -> u8 {            
      let mut numbers = [false, ..10]; 
      let mut nb = 0i;
      
      fn get_empty_number(ns: &[bool, ..10]) -> u8 {
         for n in range(1, 10){
            if !ns[n] {
               return n as u8;
            }
         }
         0
      }
   
      // First checks if the column of the given position is valid.
      for i in range(0, 9) {
         let p = self.get(i, pos.j);
         if p == 0 {
            continue;
         }
         if !numbers[p as uint] {
            numbers[p as uint] = true;
            nb += 1;
         }
      }
      
      // First checks if the row of the given position is valid.
      for j in range(0, 9) {
         let p = self.get(pos.i, j);
         if p == 0 {
            continue;
         }
         if !numbers[p as uint] {
            numbers[p as uint] = true;
            nb += 1;
         }
      }
      
      // Then check if the 3x3 square is valid.
      let i_top = if pos.i <= 2 { 0 } else if pos.i <= 5 { 3 } else { 6 };
      let j_top = if pos.j <= 2 { 0 } else if pos.j <= 5 { 3 } else { 6 };      
      for i in range(i_top, i_top + 3) {
         for j in range(j_top, j_top + 3) {
            let p = self.get(i, j);
            if p == 0 {
               continue;
            }
            if !numbers[p as uint] {               
               numbers[p as uint] = true;
               nb += 1;
            }
         }
      }
      
      if nb == 8 {
         return get_empty_number(&numbers);
      } 
      
      if nb == 9 {
         return 0;
      }
      
      for n in range(0, 9){
         if !numbers[n] {
            // Column.
            let mut valid = false;
            for i in range(0, 9) {
               if i == pos.i || self.get(i, pos.j) != 0 {
                  continue;
               }               
               if self.is_position_valid_with(&Pos { i: i, j: pos.j }, n as u8) {               
                  valid = true;
                  break;
               }
            }
            if !valid {
               return n as u8;
            }
            
            // Row.
            let mut valid = false;
            for j in range(0, 9) {
               if j == pos.j || self.get(pos.i, j) != 0 {
                  continue;
               }               
               if self.is_position_valid_with(&Pos { i: pos.i, j: j }, n as u8) {        
                  valid = true;
                  break;
               }
            }
            if !valid {
               return n as u8;
            }       
            
            // Square.
            let mut valid = false;
            let i_top = if pos.i <= 2 { 0 } else if pos.i <= 5 { 3 } else { 6 };
            let j_top = if pos.j <= 2 { 0 } else if pos.j <= 5 { 3 } else { 6 };      
            for i in range(i_top, i_top + 3) {
               for j in range(j_top, j_top + 3) {
                  if i == pos.i && j == pos.j || self.get(i, j) != 0 {
                     continue;
                  }          
                  if self.is_position_valid_with(&Pos { i: i, j: j }, n as u8) {
                     valid = true;
                     break;
                  }
               }
            }
            
            if !valid {
               return n as u8;
            }
         }
      }
      
      0u8
   }
   
   
   fn is_position_valid(&self, pos: &Pos) -> bool {
      self.is_position_valid_with(pos, self.get_pos(pos))
   }
      
   fn is_position_valid_with(&self, pos: &Pos, n: u8) -> bool {      
      // First checks if the column of the given position is valid.      
      for i in range(0, 9) {
         if i == pos.i {
            continue;
         }
         let p = self.get(i, pos.j);
         if p != 0 && p == n {
            return false;
         }
      }
      
      // First checks if the row of the given position is valid.
      for j in range(0, 9) {
         if j == pos.j {
            continue;
         }
         let p = self.get(pos.i, j);
         if p != 0 && p == n {
            return false;
         }
      }
      
      // Then check if the 3x3 square is valid.
      let i_top = if pos.i <= 2 { 0 } else if pos.i <= 5 { 3 } else { 6 };
      let j_top = if pos.j <= 2 { 0 } else if pos.j <= 5 { 3 } else { 6 };
      
      for i in range(i_top, i_top + 3) {
         for j in range(j_top, j_top + 3) {
            if i == pos.i && j == pos.j {
               continue;            
            }
            let p = self.get(i, j);
            if p != 0 && p == n {
               return false;
            }
         } 
      }
      true
   }
   
   fn pre_solve_with_constraints(&mut self) {
      let mut current_pos = Pos { i: 0, j: -1 };      
      let mut number_inserted = false;
         
      loop {         
         match self.next_free(current_pos) {
            Some(p) => current_pos = p,
            None if number_inserted => {
               number_inserted = false;
               current_pos = Pos { i: 0, j: 0 };
               continue
            },
            _ => 
               return
         }
            
         let v = self.the_only_valid_number(&current_pos);
         
         if v != 0 {
            self.set_pos(v, &current_pos);
            number_inserted = true;
         }
      }
   }   
   
   fn solve(&mut self)
   {
      self.pre_solve_with_constraints();   
   
      let mut current_pos = Pos { i: 0, j: -1 };
      let mut previous_positions: Vec<Pos> = vec![];
      
      loop {        
         match self.next_free(current_pos) {
            Some(p) => current_pos = p,
            None => return
         }
         
         loop {         
            let n = self.get_pos(&current_pos) + 1;
            self.set_pos(n, &current_pos);
            if self.is_position_valid(&current_pos) {
               previous_positions.push(current_pos);
               break;
            }
            
            while self.get_pos(&current_pos) == 9 {
               self.set_pos(0, &current_pos);
               match previous_positions.pop() {
                  Some(p) => current_pos = p,
                  None => return
               }
            }
         }     
      }
   }
}

fn main() {
   if os::args().len() != 2 {
      return print_usage(os::args()[0].as_slice());
   }
   
   let mut n = 1i;   
   let mut source = io::BufferedReader::new(io::File::open(&Path::new(os::args()[1].as_slice())));
   
   loop {
      match Board::read(&mut source) {
         Ok(ref mut board) => {
            println!("{}) #############", n);
            n += 1;
            
            match board.write(&mut io::stdio::stdout()) {
               Ok(_) => (),
               Err(e) => {
                  println!("An error occurs when writing the initial state: {}", e);
               }
            }
         
            board.solve();
            
            println!("vvvvvvvvvvv");
            
            match board.write(&mut io::stdio::stdout()) {
               Ok(_) => (),
               Err(e) => {
                  println!("An error occurs when writing the result: {}", e);
               }
            }
         },
         Err(_) =>
            return
      }
   }
}
