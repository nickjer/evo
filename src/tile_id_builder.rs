use crate::position::Position;
use crate::step::Step;
use crate::tiles::TileId;

#[derive(Debug, Clone)]
pub struct TileIdBuilder {
    position: Position,
    steps: Vec<Step>,
    x_size: usize,
    y_size: usize,
}

impl TileIdBuilder {
    pub fn new(position: Position, x_size: usize, y_size: usize) -> Self {
        assert!(position.x() < x_size);
        assert!(position.y() < y_size);
        TileIdBuilder {
            position,
            steps: Vec::new(),
            x_size,
            y_size,
        }
    }

    pub fn right(mut self, steps: usize) -> Self {
        self.steps
            .extend(std::iter::repeat(Step::Right).take(steps));
        self
    }

    pub fn left(mut self, steps: usize) -> Self {
        self.steps.extend(std::iter::repeat(Step::Left).take(steps));
        self
    }

    pub fn up(mut self, steps: usize) -> Self {
        self.steps.extend(std::iter::repeat(Step::Up).take(steps));
        self
    }

    pub fn down(mut self, steps: usize) -> Self {
        self.steps.extend(std::iter::repeat(Step::Down).take(steps));
        self
    }

    fn increment(&self, value: usize, size: usize) -> usize {
        if value == size - 1 {
            0
        } else {
            value + 1
        }
    }

    fn decrement(&self, value: usize, size: usize) -> usize {
        if value == 0 {
            size - 1
        } else {
            value - 1
        }
    }

    pub fn build(self) -> TileId {
        let new_position = self
            .steps
            .iter()
            .fold(self.position, |position, step| match step {
                Step::Left => {
                    Position::new(self.decrement(position.x(), self.x_size), position.y())
                }
                Step::Right => {
                    Position::new(self.increment(position.x(), self.x_size), position.y())
                }
                Step::Up => Position::new(position.x(), self.increment(position.y(), self.y_size)),
                Step::Down => {
                    Position::new(position.x(), self.decrement(position.y(), self.y_size))
                }
            });
        TileId::from(new_position.x() * self.y_size + new_position.y())
    }
}
