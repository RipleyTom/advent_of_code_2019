#[derive(Clone)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon { pos: [x, y, z], vel: [0, 0, 0] }
    }

    fn _print(&self) {
        println!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.pos[0], self.pos[1], self.pos[2], self.vel[0], self.vel[1], self.vel[2]
        );
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

    fn energy(&self) -> i64 {
        let pos_energy: i64 = self.pos.iter().map(|v| if *v < 0 { -*v } else { *v }).sum();
        let vel_energy: i64 = self.vel.iter().map(|v| if *v < 0 { -*v } else { *v }).sum();

        pos_energy * vel_energy
    }
}

pub fn run_puzzle() {
    let mut moons = Vec::new();

    moons.push(Moon::new(-3, 15, -11));
    moons.push(Moon::new(3, 13, -19));
    moons.push(Moon::new(-13, 18, -2));
    moons.push(Moon::new(6, 0, -1));

    for _ in 0..1000 {
        // moons.iter().for_each(|m| m.print());

        let old_moons = moons.clone();

        for m in &mut moons {
            for om in &old_moons {
                m.adjust_vel(om);
            }
        }

        moons.iter_mut().for_each(|m| m.adjust_pos());
    }

    let total_energy: i64 = moons.iter().map(|m| m.energy()).sum();
    println!("Total energy: {}", total_energy);
}
