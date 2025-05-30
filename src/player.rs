use raylib::prelude::*;
use crate::drawable::Drawable;

pub struct Player {
    position: Vector2,      // Position X (fixe), Y (variable pendant le saut)
    size: Vector2,         // Taille du joueur
    velocity_y: f32,       // Vitesse verticale pour le saut
    is_jumping: bool,      // État de saut
    is_charging_jump: bool, // État de charge du saut
    jump_charge_time: f32,  // Temps actuel de charge du saut
}

impl Player {
    pub fn new(position: Vector2, size: Vector2) -> Self {
        Player {
            position,
            size,
            velocity_y: 0.0,
            is_jumping: false,
            is_charging_jump: false,
            jump_charge_time: 0.0,
        }
    }

    pub fn update(
        &mut self,
        delta_time: f32,
        ground_level: f32,
        jump_height: f32,
        jump_charge_time_max: f32,
        gravity: f32,
        is_space_pressed: bool,
    ) {
        // Gestion du saut avec charge
        if !self.is_jumping {
            if is_space_pressed {
                if !self.is_charging_jump {
                    self.is_charging_jump = true;
                    self.jump_charge_time = 0.0;
                } else {
                    self.jump_charge_time = (self.jump_charge_time + delta_time).min(jump_charge_time_max);
                }
            } else if self.is_charging_jump {
                self.is_charging_jump = false;
                let jump_power = jump_height * (self.jump_charge_time / jump_charge_time_max);
                self.velocity_y = -f32::sqrt(2.0 * gravity * jump_power);
                self.is_jumping = true;
            }
        }

        // Appliquer la gravité et mettre à jour la position verticale
        if self.is_jumping {
            self.velocity_y += gravity * delta_time;
            self.position.y += self.velocity_y * delta_time;

            // Vérifier si le joueur est retombé au sol
            if self.position.y >= ground_level {
                self.position.y = ground_level;
                self.velocity_y = 0.0;
                self.is_jumping = false;
            }
        }
    }

    pub fn draw_jump_charge(&self, d: &mut RaylibDrawHandle, jump_charge_time_max: f32) {
        if self.is_charging_jump {
            let charge_percent = self.jump_charge_time / jump_charge_time_max;
            let bar_width = 200;
            let filled_width = (bar_width as f32 * charge_percent) as i32;

            d.draw_rectangle(10, 60, bar_width, 20, Color::GRAY);
            d.draw_rectangle(10, 60, filled_width, 20, Color::YELLOW);
            d.draw_text(
                &format!("Puissance: {:.0}%", charge_percent * 100.0),
                10,
                85,
                20,
                Color::WHITE,
            );
        }
    }
}

impl Drawable for Player {
    fn draw(&self, d: &mut RaylibDrawHandle, screen_x: f32, _screen_width: f32) {
        let x = screen_x as i32;
        let y = self.position.y as i32;
        let width = self.size.x as i32;
        let height = self.size.y as i32;

        // Couleurs du joueur
        let body_color = Color::new(30, 90, 150, 255);         // Bleu plus foncé
        let body_outline = Color::new(20, 60, 100, 255);       // Contour plus foncé
        let skin_color = Color::new(255, 213, 170, 255);       // Couleur chair
        let skin_outline = Color::new(200, 160, 130, 255);     // Contour chair
        let pants_color = Color::new(40, 40, 120, 255);        // Bleu foncé pour pantalon
        let shoes_color = Color::new(160, 40, 40, 255);        // Rouge foncé pour chaussures
        let hair_color = Color::new(80, 50, 20, 255);          // Brun pour cheveux

        // Ombre
        let ground_y = (y + height) as f32;
        d.draw_circle(
            x,
            ground_y as i32,
            (width / 2) as f32,
            Color::new(0, 0, 0, 50)
        );

        // Jambes
        // Pantalon (gauche)
        d.draw_rectangle(
            x - width / 3 - 2,
            y + height * 1/2,
            width / 3,
            height / 2,
            pants_color
        );
        // Pantalon (droite)
        d.draw_rectangle(
            x + 2,
            y + height * 1/2,
            width / 3,
            height / 2,
            pants_color
        );

        // Chaussures
        d.draw_rectangle(
            x - width / 3 - 4,
            y + height - 12,
            width / 3 + 2,
            12,
            shoes_color
        );
        d.draw_rectangle(
            x + 2,
            y + height - 12,
            width / 3 + 2,
            12,
            shoes_color
        );

        // Corps (torse)
        d.draw_rectangle(
            x - width / 2,
            y - height / 4,
            width,
            height * 3/4,
            body_color
        );

        // Contour du corps
        d.draw_rectangle_lines(
            x - width / 2,
            y - height / 4,
            width,
            height * 3/4,
            body_outline
        );

        // Tête
        d.draw_rectangle(
            x - width / 2 + 2,
            y - height / 4 - height / 4,
            width - 4,
            height / 4,
            skin_color
        );

        // Contour de tête
        d.draw_rectangle_lines(
            x - width / 2 + 2,
            y - height / 4 - height / 4,
            width - 4,
            height / 4,
            skin_outline
        );

        // Cheveux
        d.draw_rectangle(
            x - width / 2 + 2,
            y - height / 4 - height / 4,
            width - 4,
            height / 8,
            hair_color
        );

        // Yeux
        d.draw_rectangle(
            x - width / 5,
            y - height / 4 - height / 6,
            width / 10,
            height / 16,
            Color::WHITE
        );
        d.draw_rectangle(
            x + width / 10,
            y - height / 4 - height / 6,
            width / 10,
            height / 16,
            Color::WHITE
        );

        // Pupilles
        d.draw_rectangle(
            x - width / 5 + 2,
            y - height / 4 - height / 6 + 2,
            width / 15,
            height / 20,
            Color::BLACK
        );
        d.draw_rectangle(
            x + width / 10 + 2,
            y - height / 4 - height / 6 + 2,
            width / 15,
            height / 20,
            Color::BLACK
        );

        // Bouche
        if self.is_jumping || self.is_charging_jump {
            // Bouche ouverte (forme de "o") pendant le saut ou la charge
            d.draw_circle(
                x,
                y - height / 4 - height / 16,
                (width / 10) as f32,
                Color::new(200, 100, 100, 255)
            );
            d.draw_circle(
                x,
                y - height / 4 - height / 16,
                (width / 15) as f32,
                Color::new(100, 10, 10, 255)
            );
        } else {
            // Bouche fermée (sourire simple)
            d.draw_line(
                x - width / 6,
                y - height / 4 - height / 16,
                x + width / 6,
                y - height / 4 - height / 16,
                Color::new(200, 100, 100, 255)
            );
        }

        // Bras (gauche)
        d.draw_rectangle(
            x - width / 2 - width / 6,
            y,
            width / 6,
            height / 2,
            body_color
        );
        d.draw_rectangle_lines(
            x - width / 2 - width / 6,
            y,
            width / 6,
            height / 2,
            body_outline
        );

        // Bras (droite)
        d.draw_rectangle(
            x + width / 2,
            y,
            width / 6,
            height / 2,
            body_color
        );
        d.draw_rectangle_lines(
            x + width / 2,
            y,
            width / 6,
            height / 2,
            body_outline
        );

        // Mains
        d.draw_rectangle(
            x - width / 2 - width / 6 - 4,
            y + height / 2 - 8,
            width / 6 + 2,
            8,
            skin_color
        );
        d.draw_rectangle(
            x + width / 2 + 2,
            y + height / 2 - 8,
            width / 6 + 2,
            8,
            skin_color
        );

        // Détails du corps - ceinture
        d.draw_line(
            x - width / 2,
            y + height / 4,
            x + width / 2,
            y + height / 4,
            Color::new(200, 180, 60, 255)
        );

        // Détails du corps - boutons ou fermeture éclair
        for i in 0..3 {
            d.draw_rectangle(
                x - 2,
                y - height / 8 + i * (height / 6),
                4,
                4,
                Color::new(220, 220, 220, 255)
            );
        }
    }

    fn get_position(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }

    fn get_layer(&self) -> i32 {
        2
    }
}