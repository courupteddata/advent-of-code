
#[derive(PartialEq)]
pub enum Rotation {
    Left,
    Right,
    Invalid,
}

impl Rotation {
    pub fn new(rotation: char) -> Rotation {
        match rotation {
            'r' => Rotation::Right,
            'R' => Rotation::Right,
            'l' => Rotation::Left,
            'L' => Rotation::Left,
            _ => Rotation::Invalid,
        }
    }
}

pub struct Rotator {
    state: u32,
    max: u32,
    special_state: u32,
    special_counter: u64,
}

impl Rotator {
    pub fn new(max: u32, state: u32) -> Rotator {
        Rotator { state, max, special_counter: 0, special_state: 0 }
    }

    pub fn default() -> Rotator {
        Rotator {state: 50, max: 99, special_counter: 0, special_state: 0}
    }

    fn bound_state(&self, adjusted_state: i64) -> u32 {
        adjusted_state.rem_euclid((self.max as i64) + 1) as u32
    }

    fn add(a: i64, b: i64) -> i64 {
        a + b
    }

    fn sub(a: i64, b: i64) -> i64 {
        a - b
    }
    fn distance_to_special_state(&self, direction: &Rotation) -> u32 {
        //Definitely can be made more efficient, I was just getting lazy
        let state = self.state as i64;

        let operator = match direction {
            Rotation::Left => Rotator::sub,
            Rotation::Right => Rotator::add,
            Rotation::Invalid => todo!("Probably want to swap this function to return a Result and err here"),
        };

        for i in 0..=self.max {
            let bounded_state = self.bound_state(operator(state, i as i64));
            if bounded_state == self.special_state {
                return i;
            }
        }

        // Should never actually get here
        panic!("This should never happen");
    }


    pub fn rotate(&mut self, distance: u32, rotation: &Rotation) -> u32 {
        if *rotation == Rotation::Invalid {
            // Invalid rotation, nothing can be done
            return self.state;
        }

        let mut temp_state = self.state as i64;

        let distance_to_state = self.distance_to_special_state(rotation);

        let adjusted_distance =
            if distance_to_state == self.special_state {
                // Don't want to double count if already at special state
                distance
            } else if distance > distance_to_state {
                // Then we can say we are at the special value and then update the special counter
                temp_state = self.special_state as i64;
                self.special_counter += 1;
                distance - distance_to_state
            } else {
                distance
        } as i64;

        match rotation {
            Rotation::Left => {
                temp_state -= adjusted_distance;
            }
            Rotation::Right => {
                temp_state += adjusted_distance;
            }
            Rotation::Invalid => {
                // Do nothing
                return self.state;
            }
        }

        // How many times temp_state is multiple
        self.special_counter += (temp_state/(self.max + 1) as i64).abs() as u64;

        if temp_state == self.special_state as i64 {
            //Special case where we end at the desired state
            self.special_counter += 1;
        }

        // Time to bound that state
        self.state = self.bound_state(temp_state);

        self.state
    }

    pub fn state(&self) -> u32 {self.state}

    pub fn special_counter(&self) -> u64 { self.special_counter }

    pub fn parse_line(&mut self, line: String) -> Result<u32, String> {
        let line = line.trim();
        if line.len() < 2 {
            return Err(String::from("Line is too short"));
        }

        let first_char = line.chars().nth(0);

        let rotation = match first_char {
            Some(first_char) => {
                Rotation::new(first_char)
            }, None =>  {
                return Err(String::from("No rotation found"));
            }
        };

        let distance =  match line[1..].parse::<u32>() {
            Ok(distance) => {distance},
            Err(_) => {return Err(String::from("Could not parse rotation amount"));}
        };

        Ok(self.rotate(distance, &rotation))
    }
}
#[cfg(test)]
mod tests {
    use crate::Rotator;

    #[test]
    fn it_works() {
        let example_input = "L68
                                   L30
                                   R48
                                   L5
                                   R60
                                   L55
                                   L1
                                   L99
                                   R14
                                   L82";

        let mut rotator = Rotator::default();
        let mut lines = example_input.lines();

        assert_eq!(rotator.state(), 50);
        assert_eq!(rotator.special_counter, 0);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 82);
        assert_eq!(rotator.special_counter, 1);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 52);
        assert_eq!(rotator.special_counter, 1);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 0);
        assert_eq!(rotator.special_counter, 2);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 95);
        assert_eq!(rotator.special_counter, 2);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 55);
        assert_eq!(rotator.special_counter, 3);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 0);
        assert_eq!(rotator.special_counter, 4);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 99);
        assert_eq!(rotator.special_counter, 4);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 0);
        assert_eq!(rotator.special_counter, 5);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 14);
        assert_eq!(rotator.special_counter, 5);
        assert_eq!(rotator.parse_line(lines.nth(0).unwrap().to_string()).unwrap(), 32);
        assert_eq!(rotator.special_counter, 6);
        assert_eq!(rotator.state(), 32);
    }
}