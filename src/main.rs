use bevy::prelude::*;
use rand::Rng;

const WIDTH: usize = 1500;
const HEIGHT: usize = 1000;
const CELL_SIZE: f32 = 6.0;

#[derive(Resource)]
struct Game {
    current: Vec<bool>,
    next: Vec<bool>,
    timer: Timer,
    paused: bool,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Game {
            current: random_world(),
            next: vec![false; WIDTH * HEIGHT],
            timer: Timer::from_seconds(0.08, TimerMode::Repeating),
            paused: false,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".into(),
                resolution: (
                    (WIDTH as f32 * CELL_SIZE) as u32,
                    (HEIGHT as f32 * CELL_SIZE) as u32,
                )
                    .into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_game, draw_game, keyboard))
        .run();
}

fn random_world() -> Vec<bool> {
    let mut rng = rand::rng();

    (0..WIDTH * HEIGHT)
        .map(|_| rng.random_bool(0.35))
        .collect()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game.paused = !game.paused;
    }

    if keys.just_pressed(KeyCode::KeyR) {
        game.current = random_world();
    }
}

fn update_game(
    time: Res<Time>,
    mut game: ResMut<Game>,
) {
    if game.paused {
        return;
    }

    game.timer.tick(time.delta());

    if !game.timer.is_finished() {
        return;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let nx =
                        (x as isize + dx)
                            .rem_euclid(WIDTH as isize)
                            as usize;

                    let ny =
                        (y as isize + dy)
                            .rem_euclid(HEIGHT as isize)
                            as usize;

                    if game.current[ny * WIDTH + nx] {
                        neighbors += 1;
                    }
                }
            }

            let idx = y * WIDTH + x;

            game.next[idx] = match (
                game.current[idx],
                neighbors,
            ) {
                (true, 2 | 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    // Generation wechseln ohne Borrow-Checker Fehler
    let next = game.next.clone();
    let current = game.current.clone();

    game.current = next;
    game.next = current;
}

fn draw_game(
    mut gizmos: Gizmos,
    game: Res<Game>,
) {
    let ox = -(WIDTH as f32 * CELL_SIZE) / 2.0;
    let oy = -(HEIGHT as f32 * CELL_SIZE) / 2.0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if !game.current[y * WIDTH + x] {
                continue;
            }

            let px = ox + x as f32 * CELL_SIZE;
            let py = oy + y as f32 * CELL_SIZE;

            gizmos.rect_2d(
                Vec2::new(px, py),
                Vec2::splat(CELL_SIZE - 1.0),
                Color::WHITE,
            );
        }
    }
}
