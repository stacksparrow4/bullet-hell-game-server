use fps_clock::FpsClock;

use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    ops,
    sync::mpsc::{channel, Sender},
    thread,
};

const PLAYER_SPEED: f32 = 8.;

const BOUND_X: f32 = 312.;
const BOUND_Y: f32 = 232.;

#[derive(Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

enum PlayerInput {
    Vel(Vec2),
    Restart,
}

struct Enemy {
    pos: Vec2,
    vel: Vec2,
}

struct SpawnDetails(usize, f32, f32, f32, f32);

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:1234").unwrap();

    let mut games: HashMap<SocketAddr, Sender<PlayerInput>> = HashMap::new();

    let spawn_details = &[
        SpawnDetails(10, BOUND_X, 0., -10., 0.),
        SpawnDetails(20, BOUND_X, 30., -10., 0.),
        SpawnDetails(30, BOUND_X, -30., -10., 0.),
        SpawnDetails(40, BOUND_X, 0., -10., 0.),
        SpawnDetails(50, -300., BOUND_Y, 0., -10.),
        SpawnDetails(50, -270., BOUND_Y, 0., -10.),
        SpawnDetails(50, -240., BOUND_Y, 0., -10.),
        SpawnDetails(50, -210., BOUND_Y, 0., -10.),
        SpawnDetails(50, -180., BOUND_Y, 0., -10.),
        SpawnDetails(50, -150., BOUND_Y, 0., -10.),
        SpawnDetails(50, -120., BOUND_Y, 0., -10.),
        SpawnDetails(50, -90., BOUND_Y, 0., -10.),
        SpawnDetails(50, -60., BOUND_Y, 0., -10.),
        SpawnDetails(50, -30., BOUND_Y, 0., -10.),
        SpawnDetails(50, 0., BOUND_Y, 0., -10.),
        SpawnDetails(70, 300., BOUND_Y, 0., -10.),
        SpawnDetails(70, 270., BOUND_Y, 0., -10.),
        SpawnDetails(70, 240., BOUND_Y, 0., -10.),
        SpawnDetails(70, 210., BOUND_Y, 0., -10.),
        SpawnDetails(70, 180., BOUND_Y, 0., -10.),
        SpawnDetails(70, 150., BOUND_Y, 0., -10.),
        SpawnDetails(70, 120., BOUND_Y, 0., -10.),
        SpawnDetails(70, 90., BOUND_Y, 0., -10.),
        SpawnDetails(70, 60., BOUND_Y, 0., -10.),
        SpawnDetails(70, 30., BOUND_Y, 0., -10.),
        SpawnDetails(70, 0., BOUND_Y, 0., -10.),
        SpawnDetails(90, -300., BOUND_Y, 0., -10.),
        SpawnDetails(90, -270., BOUND_Y, 0., -10.),
        SpawnDetails(90, -240., BOUND_Y, 0., -10.),
        SpawnDetails(90, -210., BOUND_Y, 0., -10.),
        SpawnDetails(90, -180., BOUND_Y, 0., -10.),
        SpawnDetails(90, -150., BOUND_Y, 0., -10.),
        SpawnDetails(90, -120., BOUND_Y, 0., -10.),
        SpawnDetails(90, -90., BOUND_Y, 0., -10.),
        SpawnDetails(90, -60., BOUND_Y, 0., -10.),
        SpawnDetails(90, -30., BOUND_Y, 0., -10.),
        SpawnDetails(90, 0., BOUND_Y, 0., -10.),
        SpawnDetails(110, 300., BOUND_Y, 0., -10.),
        SpawnDetails(110, 270., BOUND_Y, 0., -10.),
        SpawnDetails(110, 240., BOUND_Y, 0., -10.),
        SpawnDetails(110, 210., BOUND_Y, 0., -10.),
        SpawnDetails(110, 180., BOUND_Y, 0., -10.),
        SpawnDetails(110, 150., BOUND_Y, 0., -10.),
        SpawnDetails(110, 120., BOUND_Y, 0., -10.),
        SpawnDetails(110, 90., BOUND_Y, 0., -10.),
        SpawnDetails(110, 60., BOUND_Y, 0., -10.),
        SpawnDetails(110, 30., BOUND_Y, 0., -10.),
        SpawnDetails(110, 0., BOUND_Y, 0., -10.),
        SpawnDetails(120, -300., BOUND_Y, 0., -10.),
        SpawnDetails(120, -270., BOUND_Y, 0., -10.),
        SpawnDetails(120, -240., BOUND_Y, 0., -10.),
        SpawnDetails(120, -210., BOUND_Y, 0., -10.),
        SpawnDetails(120, -180., BOUND_Y, 0., -10.),
        SpawnDetails(120, -150., BOUND_Y, 0., -10.),
        SpawnDetails(120, -120., BOUND_Y, 0., -10.),
        SpawnDetails(120, -90., BOUND_Y, 0., -10.),
        SpawnDetails(120, -60., BOUND_Y, 0., -10.),
        SpawnDetails(120, -30., BOUND_Y, 0., -10.),
        SpawnDetails(120, 0., BOUND_Y, 0., -10.),
        SpawnDetails(130, 300., BOUND_Y, 0., -10.),
        SpawnDetails(130, 270., BOUND_Y, 0., -10.),
        SpawnDetails(130, 240., BOUND_Y, 0., -10.),
        SpawnDetails(130, 210., BOUND_Y, 0., -10.),
        SpawnDetails(130, 180., BOUND_Y, 0., -10.),
        SpawnDetails(130, 150., BOUND_Y, 0., -10.),
        SpawnDetails(130, 120., BOUND_Y, 0., -10.),
        SpawnDetails(130, 90., BOUND_Y, 0., -10.),
        SpawnDetails(130, 60., BOUND_Y, 0., -10.),
        SpawnDetails(130, 30., BOUND_Y, 0., -10.),
        SpawnDetails(130, 0., BOUND_Y, 0., -10.),
        SpawnDetails(140, -300., BOUND_Y, 0., -10.),
        SpawnDetails(140, -270., BOUND_Y, 0., -10.),
        SpawnDetails(140, -240., BOUND_Y, 0., -10.),
        SpawnDetails(140, -210., BOUND_Y, 0., -10.),
        SpawnDetails(140, -180., BOUND_Y, 0., -10.),
        SpawnDetails(140, -150., BOUND_Y, 0., -10.),
        SpawnDetails(140, -120., BOUND_Y, 0., -10.),
        SpawnDetails(140, -90., BOUND_Y, 0., -10.),
        SpawnDetails(140, -60., BOUND_Y, 0., -10.),
        SpawnDetails(140, -30., BOUND_Y, 0., -10.),
        SpawnDetails(140, 0., BOUND_Y, 0., -10.),
        SpawnDetails(150, 300., BOUND_Y, 0., -10.),
        SpawnDetails(150, 270., BOUND_Y, 0., -10.),
        SpawnDetails(150, 240., BOUND_Y, 0., -10.),
        SpawnDetails(150, 210., BOUND_Y, 0., -10.),
        SpawnDetails(150, 180., BOUND_Y, 0., -10.),
        SpawnDetails(150, 150., BOUND_Y, 0., -10.),
        SpawnDetails(150, 120., BOUND_Y, 0., -10.),
        SpawnDetails(150, 90., BOUND_Y, 0., -10.),
        SpawnDetails(150, 60., BOUND_Y, 0., -10.),
        SpawnDetails(150, 30., BOUND_Y, 0., -10.),
        SpawnDetails(150, 0., BOUND_Y, 0., -10.),
        SpawnDetails(200, -330., BOUND_Y, 0., -10.),
        SpawnDetails(200, -300., BOUND_Y, 0., -10.),
        SpawnDetails(200, -270., BOUND_Y, 0., -10.),
        SpawnDetails(200, -240., BOUND_Y, 0., -10.),
        SpawnDetails(200, -210., BOUND_Y, 0., -10.),
        SpawnDetails(200, -180., BOUND_Y, 0., -10.),
        SpawnDetails(200, -150., BOUND_Y, 0., -10.),
        SpawnDetails(200, -120., BOUND_Y, 0., -10.),
        SpawnDetails(200, -90., BOUND_Y, 0., -10.),
        SpawnDetails(200, -60., BOUND_Y, 0., -10.),
        SpawnDetails(200, -30., BOUND_Y, 0., -10.),
        SpawnDetails(200, 0., BOUND_Y, 0., -10.),
        SpawnDetails(200, 330., BOUND_Y, 0., -10.),
        SpawnDetails(200, 300., BOUND_Y, 0., -10.),
        SpawnDetails(200, 270., BOUND_Y, 0., -10.),
        SpawnDetails(200, 240., BOUND_Y, 0., -10.),
        SpawnDetails(200, 210., BOUND_Y, 0., -10.),
        SpawnDetails(200, 180., BOUND_Y, 0., -10.),
        SpawnDetails(200, 150., BOUND_Y, 0., -10.),
        SpawnDetails(200, 120., BOUND_Y, 0., -10.),
        SpawnDetails(200, 90., BOUND_Y, 0., -10.),
        SpawnDetails(200, 60., BOUND_Y, 0., -10.),
        SpawnDetails(200, 30., BOUND_Y, 0., -10.),
        SpawnDetails(200, 0., BOUND_Y, 0., -10.),
        SpawnDetails(230, 0., BOUND_X, 0., -3.),
    ];

    let win_time = 300;

    println!("Game server up!");

    loop {
        let mut buf = [0; 2048];
        if let Ok((amt, src)) = socket.recv_from(&mut buf) {
            if amt >= 2048 {
                eprintln!("WARNING: packet too big!!");
            }

            match buf[0] {
                1 => {
                    if let Some(tx) = games.get_mut(&src) {
                        let mut stray = false;

                        if buf[9] == 0 {
                            let dx = f32::from_be_bytes((&buf[1..5]).try_into().unwrap());
                            let dy = f32::from_be_bytes((&buf[5..9]).try_into().unwrap());

                            if dx.abs() > 1.1f32 || dy.abs() > 1.1f32 {
                                eprintln!("CHEATING!");
                                continue;
                            }

                            let new_vel = Vec2::new(dx, dy) * PLAYER_SPEED;

                            if let Err(_) = tx.send(PlayerInput::Vel(new_vel)) {
                                stray = true;
                            }
                        } else {
                            if let Err(_) = tx.send(PlayerInput::Restart) {
                                stray = true;
                            }
                        }

                        if stray {
                            println!("Removing stray game");
                            games.remove(&src);
                        }
                    } else {
                        println!("Starting game");

                        let socket_clone = socket.try_clone().unwrap();

                        let (tx, rx) = channel();

                        thread::spawn(move || {
                            let mut fps = FpsClock::new(30);

                            let mut player_pos = Vec2::new(0., 0.);
                            let mut player_vel = Vec2::new(0., 0.);
                            let mut dead = false;

                            let mut enemies: Vec<Enemy> = Vec::new();

                            let mut frames_without_interaction = 0usize;

                            let mut frame = 0;

                            let mut spawn_iter = spawn_details.iter().peekable();

                            loop {
                                fps.tick();

                                // RECIEVE MESSAGES
                                frames_without_interaction += 1;
                                while let Ok(data) = rx.try_recv() {
                                    match data {
                                        PlayerInput::Vel(vel) => {
                                            player_vel = vel;
                                        }
                                        PlayerInput::Restart => {
                                            player_pos = Vec2::new(0., 0.);
                                            player_vel = Vec2::new(0., 0.);
                                            dead = false;
                                            frame = 0;
                                            spawn_iter = spawn_details.iter().peekable();
                                            enemies = Vec::new();
                                        }
                                    }
                                    frames_without_interaction = 0;
                                }

                                if frames_without_interaction > 60 {
                                    println!("Closing game");
                                    break;
                                }

                                // UPDATE GAME
                                if player_pos.x.abs() >= BOUND_X || player_pos.y.abs() >= BOUND_Y {
                                    dead = true;
                                }

                                if !dead {
                                    player_pos += player_vel;
                                }

                                while let Some(curr_enemy) = spawn_iter.peek() {
                                    if curr_enemy.0 <= frame {
                                        enemies.push(Enemy {
                                            pos: Vec2::new(curr_enemy.1, curr_enemy.2),
                                            vel: Vec2::new(curr_enemy.3, curr_enemy.4),
                                        });
                                        spawn_iter.next();
                                    } else {
                                        break;
                                    }
                                }

                                for enem in enemies.iter_mut() {
                                    enem.pos += enem.vel;

                                    let delta = enem.pos - player_pos;
                                    if delta.x.abs() < 30f32 && delta.y.abs() < 30f32 {
                                        dead = true;
                                    }
                                }

                                // SEND PACKET
                                let mut pack = vec![1];

                                pack.extend_from_slice(&player_pos.x.to_be_bytes());
                                pack.extend_from_slice(&player_pos.y.to_be_bytes());

                                pack.extend_from_slice(&player_vel.x.to_be_bytes());
                                pack.extend_from_slice(&player_vel.y.to_be_bytes());

                                pack.push(if dead { 1 } else { 0 });

                                for enemy in enemies.iter() {
                                    pack.extend_from_slice(&enemy.pos.x.to_be_bytes());
                                    pack.extend_from_slice(&enemy.pos.y.to_be_bytes());

                                    pack.extend_from_slice(&enemy.vel.x.to_be_bytes());
                                    pack.extend_from_slice(&enemy.vel.y.to_be_bytes());
                                }

                                socket_clone.send_to(&pack, src).unwrap();

                                if !dead && frame > win_time {
                                    let pack_2 = b"\x02CTF{i_wasted_ur_time?}";

                                    socket_clone.send_to(pack_2, src).unwrap();
                                }

                                frame += 1;
                            }
                        });

                        games.insert(src, tx);
                    }
                }
                _ => {
                    println!("Unknown header: {}", buf[0]);
                }
            };
        }
    }
}
