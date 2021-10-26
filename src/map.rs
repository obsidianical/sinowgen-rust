struct Map<T> {
    size: [u32; 2],
    map: Vec<T>,
}

impl Map<T> {
    pub(crate) fn new<T>(size: [u32; 2]) -> Map<T> {
        Map {
            size,
            map: vec![],
        }
    }

    pub(crate) fn set_pos(&mut self, pos: [u32; 2], value: T) {
        self.map[get_index(pos)] = value;
    }
    pub(crate) fn get_pos(&self, pos: [u32; 2]) {
        self.map[get_index(pos)]
    }

    fn get_index(&self, pos: [u32; 2]) -> usize {
        (self.size[0] * pos[0] + pos[1]) as usize
    }
}

impl Copy for Map<T> {}

impl Clone for Map<T> {
    fn clone(&self) -> Self {
        *self
    }
}


fn get_position(width: u32, index: usize) -> [u32; 2] {
    let y = (index % width) as u32;
    [((index - y) / width) as u32, y]
}
