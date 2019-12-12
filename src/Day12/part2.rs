use num::integer::lcm;

#[derive(Clone)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon { pos: [x, y, z], vel: [0, 0, 0] }
    }

    fn adjust_vel(&mut self, om: &Moon) {
        for i in 0..3 {
            if self.pos[i] != om.pos[i] {
                if self.pos[i] > om.pos[i] {
                    self.vel[i] -= 1;
                } else {
                    self.vel[i] += 1;
                }
            }
        }
    }

    fn adjust_pos(&mut self) {
        for i in 0..3 {
            self.pos[i] += self.vel[i];
        }
    }
}

pub fn run_puzzle() {
    let mut moons = Vec::new();

    moons.push(Moon::new(-3, 15, -11));
    moons.push(Moon::new(3, 13, -19));
    moons.push(Moon::new(-13, 18, -2));
    moons.push(Moon::new(6, 0, -1));

    let mut periods = [0; 3];

    let initial_state = moons.clone();

    for round in 0..1000000 {
        for coor in 0..3 {
            if periods[coor] == 0 {
                let mut found = true;
                for i in 0..4 {
                    if moons[i].pos[coor] != initial_state[i].pos[coor] || moons[i].vel[coor] != initial_state[i].vel[coor] {
                        found = false;
                        break;
                    }
                }
                if found {
                    periods[coor] = round;
                }
            }
        }

        if periods[0] != 0 && periods[1] != 0 && periods[2] != 0 {
            break;
        }

        let old_moons = moons.clone();

        for m in &mut moons {
            for om in &old_moons {
                m.adjust_vel(om);
            }
        }

        moons.iter_mut().for_each(|m| m.adjust_pos());
    }

    let result = lcm::<i64>(lcm::<i64>(periods[0], periods[1]), periods[2]);

    println!("First repetition: {}", result);
}
