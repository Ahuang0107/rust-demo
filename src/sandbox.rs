pub const SIMULATION_WIDTH: usize = 400;
pub const SIMULATION_HEIGHT: usize = 400;

pub struct Sandbox {
    pub cells: [[Option<Particle>; SIMULATION_HEIGHT]; SIMULATION_WIDTH],
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub updated: bool,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            cells: [[None; SIMULATION_HEIGHT]; SIMULATION_WIDTH],
        }
    }
    pub fn update(&mut self) {
        for x in 0..SIMULATION_WIDTH {
            for y in 0..SIMULATION_HEIGHT {
                if let Some(particle) = &mut self.cells[x][y] {
                    particle.updated = false;
                }
            }
        }

        for x in (0..SIMULATION_WIDTH).rev() {
            for y in (0..SIMULATION_HEIGHT).rev() {
                if let Some(particle) = &self.cells[x][y] {
                    if !particle.updated {
                        let mut new_particle_position = Some((x, y));
                        loop {
                            if y != SIMULATION_HEIGHT - 1 {
                                if self.cells[x][y + 1].is_none() {
                                    self.cells[x][y + 1] = self.cells[x][y].take();
                                    new_particle_position = Some((x, y + 1));
                                    break;
                                }
                                if x != 0 {
                                    if self.cells[x - 1][y + 1].is_none() {
                                        self.cells[x - 1][y + 1] = self.cells[x][y].take();
                                        new_particle_position = Some((x - 1, y + 1));
                                        break;
                                    }
                                }
                                if x != SIMULATION_WIDTH - 1 {
                                    if self.cells[x + 1][y + 1].is_none() {
                                        self.cells[x + 1][y + 1] = self.cells[x][y].take();
                                        new_particle_position = Some((x + 1, y + 1));
                                        break;
                                    }
                                }
                            }
                            break;
                        }
                        // TODO
                        if let Some((x, y)) = new_particle_position {
                            self.cells[x][y].as_mut().unwrap().updated = true;
                        }
                    }
                }
            }
        }
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        let mut i = 0;
        for y in 0..SIMULATION_HEIGHT {
            for x in 0..SIMULATION_WIDTH {
                if let Some(_) = &self.cells[x][y] {
                    frame[i] = 196;
                    frame[i + 1] = 192;
                    frame[i + 2] = 135;
                    frame[i + 3] = 255;
                } else {
                    frame[i + 3] = 0;
                }
                i += 4;
            }
        }
    }
}

impl Particle {
    pub fn new() -> Self {
        Self { updated: false }
    }
}
