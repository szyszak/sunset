use rand::Rng;

#[derive(Debug)]
pub enum Direction {
    Forwards,
    Backwards,
}

impl Direction {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        if rng.gen::<bool>() {
            Direction::Forwards
        } else {
            Direction::Backwards
        }
    }
}

pub enum StarOrigin {
    Side,
    Bottom,
}

impl StarOrigin {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();

        if rng.gen::<bool>() {
            StarOrigin::Side
        } else {
            StarOrigin::Bottom
        }
    }
}
