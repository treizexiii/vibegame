use raylib::prelude::*;
use rand::Rng;
use crate::drawable::Drawable;

pub struct Obstacle {
    position: Vector2,
    size: Vector2,
    obstacle_type: ObstacleType,
}

pub enum ObstacleType {
    Spike,
    Block,
    FloatingPlatform,
}

impl Obstacle {
    pub fn new(position: Vector2, obstacle_type: ObstacleType) -> Self {
        let size = match obstacle_type {
            ObstacleType::Spike => Vector2::new(30.0, 30.0),
            ObstacleType::Block => Vector2::new(40.0, 40.0),
            ObstacleType::FloatingPlatform => Vector2::new(100.0, 20.0),
        };

        Obstacle {
            position,
            size,
            obstacle_type,
        }
    }

    pub fn collides_with(&self, player_pos: Vector2, player_size: Vector2) -> bool {
        // Vérification simple de collision par boîtes englobantes
        let player_left = player_pos.x - player_size.x / 2.0;
        let player_right = player_pos.x + player_size.x / 2.0;
        let player_top = player_pos.y - player_size.y / 2.0;
        let player_bottom = player_pos.y + player_size.y / 2.0;

        let obstacle_left = self.position.x;
        let obstacle_right = self.position.x + self.size.x;
        let obstacle_top = self.position.y;
        let obstacle_bottom = self.position.y + self.size.y;

        player_left < obstacle_right &&
        player_right > obstacle_left &&
        player_top < obstacle_bottom &&
        player_bottom > obstacle_top
    }
}

impl Drawable for Obstacle {
    fn draw(&self, d: &mut RaylibDrawHandle, screen_x: f32, screen_width: f32) {
        // Ne dessiner que si l'obstacle est visible à l'écran
        if screen_x < -self.size.x || screen_x > screen_width {
            return;
        }

        match self.obstacle_type {
            ObstacleType::Spike => {
                // Dessiner un triangle pointu
                d.draw_triangle(
                    Vector2::new(screen_x, self.position.y + self.size.y),
                    Vector2::new(screen_x + self.size.x, self.position.y + self.size.y),
                    Vector2::new(screen_x + self.size.x / 2.0, self.position.y),
                    Color::RED,
                );
            },
            ObstacleType::Block => {
                // Dessiner un bloc avec un contour
                d.draw_rectangle(
                    screen_x as i32,
                    self.position.y as i32,
                    self.size.x as i32,
                    self.size.y as i32,
                    Color::GRAY,
                );
                d.draw_rectangle_lines(
                    screen_x as i32,
                    self.position.y as i32,
                    self.size.x as i32,
                    self.size.y as i32,
                    Color::DARKGRAY,
                );
            },
            ObstacleType::FloatingPlatform => {
                // Dessiner une plateforme flottante
                d.draw_rectangle(
                    screen_x as i32,
                    self.position.y as i32,
                    self.size.x as i32,
                    self.size.y as i32,
                    Color::DARKGREEN,
                );
                // Ajouter des détails
                for i in 0..3 {
                    d.draw_line(
                        (screen_x + i as f32 * 30.0) as i32,
                        self.position.y as i32,
                        (screen_x + i as f32 * 30.0) as i32,
                        (self.position.y + self.size.y) as i32,
                        Color::GREEN,
                    );
                }
            },
        }
    }

    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }

    fn get_layer(&self) -> i32 {
        1
    }
}

pub struct ObstacleManager {
    obstacles: Vec<Obstacle>,
    last_spawn_position: f32,
    min_distance_between_obstacles: f32,
}

impl ObstacleManager {
    pub fn new() -> Self {
        ObstacleManager {
            obstacles: Vec::new(),
            last_spawn_position: 0.0,
            min_distance_between_obstacles: 300.0,
        }
    }

    pub fn update(&mut self, camera_offset: f32, screen_width: f32, ground_level: f32) {
        let mut rng = rand::thread_rng();
        
        // Supprimer les obstacles qui sont trop loin derrière la caméra
        self.obstacles.retain(|obstacle| {
            obstacle.position.x + obstacle.size.x > camera_offset - screen_width / 2.0
        });

        // Vérifier si nous devons générer un nouvel obstacle
        let current_position = camera_offset + screen_width;
        if current_position - self.last_spawn_position >= self.min_distance_between_obstacles {
            // Décider aléatoirement quel type d'obstacle créer
            let obstacle_type = match rng.random_range(0..3) {
                0 => ObstacleType::Spike,
                1 => ObstacleType::Block,
                _ => ObstacleType::FloatingPlatform,
            };

            // Déterminer la taille de l'obstacle
            let size = match obstacle_type {
                ObstacleType::Spike => Vector2::new(30.0, 30.0),
                ObstacleType::Block => Vector2::new(40.0, 40.0),
                ObstacleType::FloatingPlatform => Vector2::new(100.0, 20.0),
            };

            // Position Y en fonction du type d'obstacle
            let y = match obstacle_type {
                // Les obstacles au sol doivent être alignés avec le niveau du sol du joueur
                ObstacleType::Spike | ObstacleType::Block => ground_level,
                // Les plateformes flottantes doivent être positionnées par rapport au même niveau
                ObstacleType::FloatingPlatform => ground_level - rng.random_range(60.0..100.0),
            };

            // Créer le nouvel obstacle
            let obstacle = Obstacle::new(
                Vector2::new(current_position, y),
                obstacle_type,
            );

            self.obstacles.push(obstacle);
            self.last_spawn_position = current_position;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, camera_offset: f32, screen_width: f32) {
        for obstacle in &self.obstacles {
            let screen_x = obstacle.position.x - camera_offset;
            obstacle.draw(d, screen_x, screen_width);
        }
    }

    pub fn check_collisions(&self, player_pos: Vector2, player_size: Vector2) -> bool {
        self.obstacles.iter().any(|obstacle| obstacle.collides_with(player_pos, player_size))
    }
} 