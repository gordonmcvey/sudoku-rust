use crate::sudoku::option_finder::{OptionFinder, OptionList, StandardOptionFinder};
use crate::sudoku::reference::GridReference;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CachedOptionFinder<'problem> {
    finder: StandardOptionFinder<'problem>,
    option_cache: HashMap<GridReference, OptionList>,
}

impl<'problem> CachedOptionFinder<'problem> {
    pub fn new(finder: StandardOptionFinder<'problem>) -> Self {
        Self {
            finder,
            option_cache: HashMap::new(),
        }
    }
}

impl<'problem> OptionFinder for CachedOptionFinder<'problem> {
    fn find_for_cell(&mut self, grid_ref: &GridReference) -> OptionList {
        if !self.option_cache.contains_key(&grid_ref) {
            self.option_cache.insert(grid_ref.clone(), self.finder.find_for_cell(&grid_ref));
        }

        self.option_cache[&grid_ref].clone()
    }
}
