#[derive(Copy, Clone, Debug)]
pub struct IndexedCharacters<'a> {
    characters: &'a Vec<char>,
    index: usize,
}

impl<'a> IndexedCharacters<'a> {
    pub fn new(characters: &'a Vec<char>) -> IndexedCharacters<'a> {
        IndexedCharacters {
            characters,
            index: 0,
        }
    }

    pub fn progress(&self) -> IndexedCharacters<'a> {
        self.jump(1)
    }

    pub fn previous_character(self) -> Option<&'a char> {
        self.characters.get(self.index - 1)
    }

    pub fn current_character(self) -> Option<&'a char> {
        self.characters.get(self.index)
    }

    pub fn get_index(self) -> usize {
        self.index
    }

    pub fn jump(&self, jump: usize) -> IndexedCharacters<'a> {
        IndexedCharacters {
            characters: &self.characters,
            index: self.index + jump,
        }
    }
}
