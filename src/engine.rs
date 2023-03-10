extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, Line, PrimitiveStyle, Rectangle,
    },
};
use uefi_graphics::UefiDisplay;

#[derive(Clone, PartialEq, Eq)]
enum ItemState {
    Selected,
    Unselected,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ItemSymbol {
    X,
    O,
    Empty,
}

#[derive(Clone)]
struct StateFullItem {
    symbol: ItemSymbol,
    state: ItemState,
}

#[derive(Clone)]
pub struct Engine {
    items: Vec<StateFullItem>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            items: vec![
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Selected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
                StateFullItem {
                    symbol: ItemSymbol::Empty,
                    state: ItemState::Unselected,
                },
            ]
        }
    }

    pub fn select_item(&mut self, index: usize) {
        // First unselect all items
        for item in self.items.iter_mut() {
            if item.state == ItemState::Selected {
                item.state = ItemState::Unselected;
            }
        }
        self.items[index].state = ItemState::Selected;
    }

    pub fn get_selected_item_symbol(&self) -> Option<ItemSymbol> {
        for item in self.items.iter() {
            if item.state == ItemState::Selected {
                return Some(item.symbol);
            }
        }
        None
    }

    pub fn set_item_symbol(&mut self, index: usize, symbol: ItemSymbol) {
        self.items[index].symbol = symbol;
    }

    pub fn get_empty_items(&self) -> Vec<usize> {
        let mut empty_items = Vec::new();
        for (index, item) in self.items.iter().enumerate() {
            if item.symbol == ItemSymbol::Empty {
                empty_items.push(index);
            }
        }
        empty_items
    }

    pub fn win(&self) -> ItemSymbol {
        // Check the rows
        for row in 0..3 {
            if self.items[row * 3].symbol == self.items[row * 3 + 1].symbol && self.items[row * 3].symbol == self.items[row * 3 + 2].symbol {
                return self.items[row * 3].symbol;
            }
        }
        // Check the columns
        for column in 0..3 {
            if self.items[column].symbol == self.items[column + 3].symbol && self.items[column].symbol == self.items[column + 6].symbol {
                return self.items[column].symbol;
            }
        }
        // Check the diagonals
        if self.items[0].symbol == self.items[4].symbol && self.items[0].symbol == self.items[8].symbol {
            return self.items[0].symbol;
        }
        if self.items[2].symbol == self.items[4].symbol && self.items[2].symbol == self.items[6].symbol {
            return self.items[2].symbol;
        }
        // No win
        ItemSymbol::Empty
    }

    pub fn check_win(&self) -> bool {
        self.win() != ItemSymbol::Empty
    }

    fn draw_items(&self, width: u32, height: u32, display: &mut UefiDisplay) {
        // Set default color
        let default_color = Rgb888::new(119, 185, 242);
        let unavailable_color = Rgb888::RED;
        let available_color = Rgb888::GREEN;
        
        // Draw the items
        for item_index in 0..self.items.len() {
            let item = &self.items[item_index];
            // Define the center point of the item
            let x = match item_index {
                0 | 3 | 6 => width / 6,
                1 | 4 | 7 => width / 2,
                2 | 5 | 8 => (5 * width) / 6,
                _ => unreachable!(),
            };
            let y = match item_index {
                0..=2 => height / 6,
                3..=5 => height / 2,
                6..=8 => (5 * height) / 6,
                _ => unreachable!(),
            };

            // Select the color to use
            let color = match item.state {
                ItemState::Selected => if item.symbol == ItemSymbol::Empty { available_color } else { unavailable_color },
                ItemState::Unselected => default_color,
            };
            // Draw the item
            match item.symbol {
                ItemSymbol::X => {
                    Line::new(Point::new((x - 50) as i32, (y + 50) as i32), Point::new((x + 50) as i32, (y - 50) as i32))
                        .into_styled(PrimitiveStyle::with_stroke(color, 1))
                        .draw(display)
                        .unwrap();
                    Line::new(Point::new((x - 50) as i32, (y - 50) as i32), Point::new((x + 50) as i32, (y + 50) as i32))
                        .into_styled(PrimitiveStyle::with_stroke(color, 1))
                        .draw(display)
                        .unwrap();
                },
                ItemSymbol::O => {
                    Circle::with_center(Point::new(x as i32, y as i32), 100)
                        .into_styled(PrimitiveStyle::with_stroke(color, 1))
                        .draw(display)
                        .unwrap();
                },
                ItemSymbol::Empty => {
                    // If selected, draw a cross
                    if item.state == ItemState::Selected {
                        Line::new(Point::new((x - 50) as i32, (y + 50) as i32), Point::new((x + 50) as i32, (y - 50) as i32))
                            .into_styled(PrimitiveStyle::with_stroke(color, 1))
                            .draw(display)
                            .unwrap();
                        Line::new(Point::new((x - 50) as i32, (y - 50) as i32), Point::new((x + 50) as i32, (y + 50) as i32))
                            .into_styled(PrimitiveStyle::with_stroke(color, 1))
                            .draw(display)
                            .unwrap();
                    }
                },
            };
        };
    }

    pub fn draw(&self, width: u32, height: u32, display: &mut UefiDisplay) {
        // Clear the screen
        display.clear(Rgb888::BLACK).unwrap();
        // Set default color
        let default_color = Rgb888::new(119, 185, 242);
        // Draw the vertical lines
        for x in [width / 3, (2 * width) / 3].iter() {
            Rectangle::new(Point::new(*x as i32, 0), Size::new(1, height as u32))
                .into_styled(PrimitiveStyle::with_stroke(default_color, 1))
                .draw(display)
                .unwrap();
        }
        // Draw the horizontal lines
        for y in [height / 3, (2 * height) / 3].iter() {
            Rectangle::new(Point::new(0, *y as i32), Size::new(width as u32, 1))
                .into_styled(PrimitiveStyle::with_stroke(default_color, 1))
                .draw(display)
                .unwrap();
        }
        // Draw the items
        self.draw_items(width, height, display);
    }
}