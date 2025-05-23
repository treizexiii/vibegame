mod player;
mod scenery;
mod drawable;
mod utils;
mod obstacle;

use drawable::Drawable;
use player::Player;
use raylib::prelude::*;
use utils::{generate_world, handle_input};
use obstacle::ObstacleManager;

const GAME_OVER_TEXT: &str = "Game Over!";
const RESTART_TEXT: &str = "Appuyez sur ESPACE pour recommencer";
const GAME_OVER_FONT_SIZE: i32 = 40;
const RESTART_FONT_SIZE: i32 = 20;

fn main() {
    // Initialisation du jeu
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Vibe Game")
        .build();

    // Définir le taux de rafraîchissement cible (FPS)
    rl.set_target_fps(60);

    let screen_width = 800.0;
    let screen_height = 600.0;

    // Constantes de jeu
    let ground_level = screen_height - 50.0 - 60.0; // Hauteur du sol
    let jump_height = 200.0; // Hauteur maximale du saut en pixels
    let jump_charge_time_max = 0.5; // Temps maximal pour charger le saut (en secondes)
    let gravity = 1200.0; // Force de gravité
    let player_speed = 200.0; // Vitesse de déplacement horizontal

    // Initialiser le joueur au milieu de l'écran et au niveau du sol
    let mut player = Player::new(
        Vector2::new(screen_width / 2.0, ground_level),
        Vector2::new(40.0, 60.0),
    );

    // Décalage de la caméra (position du monde)
    let mut camera_offset = 0.0;

    // Générer le monde
    let scenery = generate_world(ground_level, screen_width);

    // Initialiser le gestionnaire d'obstacles
    let mut obstacle_manager = ObstacleManager::new();

    // Variables de jeu
    let mut game_over = false;
    let mut score = 0;

    // Boucle principale du jeu
    while !rl.window_should_close() {
        // Récupérer le delta time
        let delta_time = rl.get_frame_time();

        // Obtenir le FPS
        let current_fps = rl.get_fps();

        if !game_over {
            // Gérer les entrées utilisateur
            let (direction, space_pressed) = handle_input(&rl);
            
            // Déplacer la caméra au lieu du joueur
            camera_offset += direction * player_speed * delta_time;

            // Mettre à jour le joueur
            player.update(
                delta_time,
                ground_level,
                jump_height,
                jump_charge_time_max,
                gravity,
                space_pressed,
            );

            // Mettre à jour les obstacles
            obstacle_manager.update(camera_offset, screen_width, ground_level);

            // Vérifier les collisions avec la position mondiale du joueur
            if obstacle_manager.check_collisions(
                Vector2::new(screen_width / 2.0 + camera_offset, player.get_position().1),
                Vector2::new(player.get_size().0, player.get_size().1)
            ) {
                game_over = true;
            }

            // Mettre à jour le score (basé sur la distance parcourue)
            score = (camera_offset / 100.0) as i32;
        }

        // Vérifier le redémarrage avant de commencer le dessin
        if game_over && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            game_over = false;
            camera_offset = 0.0;
            score = 0;
            player = Player::new(
                Vector2::new(screen_width / 2.0, ground_level),
                Vector2::new(40.0, 60.0),
            );
            obstacle_manager = ObstacleManager::new();
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::SKYBLUE);

        // Dessiner sol
        d.draw_rectangle(
            0,
            ground_level as i32,
            screen_width as i32,
            (screen_height - ground_level) as i32,
            Color::GREEN,
        );

        // Dessiner les éléments de décor (en fonction de la position de la caméra)
        scenery.draw(&mut d, camera_offset, screen_width);

        // Dessiner les obstacles
        obstacle_manager.draw(&mut d, camera_offset, screen_width);

        // Dessiner le joueur
        player.draw(&mut d, player.get_position().0, screen_width);

        // Afficher les informations
        d.draw_text(&format!("FPS: {}", current_fps), 10, 10, 20, Color::WHITE);
        d.draw_text(
            &format!("Score: {}", score),
            10,
            30,
            20,
            Color::WHITE,
        );

        // Afficher la barre de charge du saut
        player.draw_jump_charge(&mut d, jump_charge_time_max);

        // Afficher l'écran de game over si nécessaire
        if game_over {
            d.draw_text(
                GAME_OVER_TEXT,
                (screen_width as i32 / 2) - 100,
                (screen_height as i32 / 2) - GAME_OVER_FONT_SIZE / 2,
                GAME_OVER_FONT_SIZE,
                Color::RED,
            );
            
            d.draw_text(
                RESTART_TEXT,
                (screen_width as i32 / 2) - 150,
                (screen_height as i32 / 2) + 50,
                RESTART_FONT_SIZE,
                Color::WHITE,
            );
        }
    }
}
