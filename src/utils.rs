use rand::{thread_rng, Rng};
use raylib::{ffi::KeyboardKey, RaylibHandle};

use crate::scenery::{create_scenery_element, Scenery, SceneryType};

pub fn handle_input(rl: &RaylibHandle) -> (f32, bool) {
    // Direction du mouvement (gauche/droite)
    let mut direction = 0.0;
    
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        direction = 1.0;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        direction = -1.0;
    }
    
    // État de la touche espace (pour le saut)
    let space_pressed = rl.is_key_down(KeyboardKey::KEY_SPACE);
    
    (direction, space_pressed)
}

// Ajoutez cette fonction pour générer le monde
pub fn generate_world(ground_level: f32, _screen_width: f32) -> Scenery {
    let mut rng = thread_rng();
    let mut scenery = Scenery::new();

    // Ajouter des arbres
    for i in 0..20 {
        let x = i as f32 * 300.0 + rng.random_range(0..100) as f32;
        let height = 100.0 + rng.random_range(50..150) as f32;

        let tree_elements = create_scenery_element(
            SceneryType::Tree,
            x,
            0.0, // Y sera calculé en fonction de ground_level
            Some((height, 0.0)),
            ground_level,
        );

        scenery.extend(tree_elements);
    }

    // Ajouter des nuages
    for i in 0..10 {
        let x = i as f32 * 400.0 + rng.random_range(0..200) as f32;
        let y = rng.random_range(50..150) as f32;
        let width = 100.0 + rng.random_range(0..50) as f32;

        let cloud_element =
            create_scenery_element(SceneryType::Cloud, x, y, Some((width, 40.0)), ground_level);

        scenery.extend(cloud_element);
    }

    // Ajouter des montagnes en arrière-plan
    for i in 0..5 {
        let x = i as f32 * 500.0;
        let height = 200.0 + rng.random_range(0..100) as f32;

        let mountain_element = create_scenery_element(
            SceneryType::Mountain,
            x,
            0.0, // Y sera calculé en fonction de ground_level
            Some((height, 0.0)),
            ground_level,
        );

        scenery.extend(mountain_element);
    }

    // Ajouter du sol/herbe
    for i in 0..30 {
        let x = i as f32 * 200.0;

        // Sol principal
        let ground_element = create_scenery_element(
            SceneryType::Ground,
            x,
            0.0, // Y sera calculé en fonction de ground_level
            None,
            ground_level,
        );

        scenery.extend(ground_element);

        // Petites touffes d'herbe aléatoires
        if rng.random_bool(0.6) {
            let grass_x = x + rng.random_range(20..180) as f32;

            let grass_element = create_scenery_element(
                SceneryType::GrassTuft,
                grass_x,
                0.0, // Y sera calculé en fonction de ground_level
                None,
                ground_level,
            );

            scenery.extend(grass_element);
        }
    }

    scenery
}