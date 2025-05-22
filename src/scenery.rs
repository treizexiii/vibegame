use raylib::prelude::*;

use crate::drawable::Drawable;

// Collection d'éléments de décor
pub struct Scenery {
    elements: Vec<Box<dyn Drawable>>,
}

impl Scenery {
    pub fn new() -> Self {
        Scenery {
            elements: Vec::new(),
        }
    }

    // Méthode pour ajouter un élément déjà construit
    // fn push(&mut self, element: Box<dyn Drawable>) {
    //     self.elements.push(element);
    // }

    // Méthode pour ajouter plusieurs éléments
    pub fn extend(&mut self, elements: Vec<Box<dyn Drawable>>) {
        for element in elements {
            self.elements.push(element);
        }
    }

    // Méthode pour dessiner tous les éléments par couche
    pub fn draw(&self, d: &mut RaylibDrawHandle, camera_offset: f32, screen_width: f32) {
        // Dessiner par couche (0 = arrière-plan, 1 = milieu, 2 = premier plan)
        for layer in 0..3 {
            for element in &self.elements {
                if element.get_layer() == layer {
                    let (x, _) = element.get_position();
                    let (width, _) = element.get_size();

                    // Différentes vitesses de parallaxe selon la couche
                    let parallax_factor = match layer {
                        0 => 0.5, // Arrière-plan
                        1 => 0.8, // Milieu
                        _ => 1.0, // Premier plan
                    };

                    let screen_x = x - camera_offset * parallax_factor;
                    
                    // Vérification plus précise avec la taille
                    if screen_x + width >= 0.0 && screen_x <= screen_width {
                        element.draw(d, screen_x, screen_width);
                    }
                }
            }
        }
    }
}

// Structure de base pour un élément de décor
pub struct SceneryElement {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    layer: i32,
}

impl SceneryElement {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: Color, layer: i32) -> Self {
        SceneryElement {
            x,
            y,
            width,
            height,
            color,
            layer,
        }
    }
}

impl Drawable for SceneryElement {
    fn draw(&self, d: &mut RaylibDrawHandle, screen_x: f32, screen_width: f32) {
        if screen_x + self.width >= 0.0 && screen_x <= screen_width {
            d.draw_rectangle(
                screen_x as i32,
                self.y as i32,
                self.width as i32,
                self.height as i32,
                self.color,
            );
        }
    }

    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn get_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn get_layer(&self) -> i32 {
        self.layer
    }
}

// Énumération des types d'éléments de décor
#[derive(Clone, Copy)]
pub enum SceneryType {
    Tree,
    Cloud,
    Mountain,
    Ground,
    GrassTuft,
}

// Fonction pour créer un élément spécifique
pub fn create_scenery_element(
    element_type: SceneryType,
    x: f32,
    y: f32,
    params: Option<(f32, f32)>,
    ground_level: f32,
) -> Vec<Box<dyn Drawable>> {
    let mut elements: Vec<Box<dyn Drawable>> = Vec::new();

    match element_type {
        SceneryType::Tree => {
            let height = params.unwrap_or((100.0, 0.0)).0;

            // Tronc d'arbre
            elements.push(Box::new(SceneryElement::new(
                x,
                ground_level - height,
                20.0,
                height,
                Color::new(139, 69, 19, 255), // Marron
                2,                            // Premier plan
            )) as Box<dyn Drawable>);

            // Feuillage
            elements.push(Box::new(SceneryElement::new(
                x - 30.0,
                ground_level - height - 60.0,
                80.0,
                80.0,
                Color::new(34, 139, 34, 255), // Vert forêt
                2,                            // Premier plan
            )) as Box<dyn Drawable>);
        }
        SceneryType::Cloud => {
            let (width, height) = params.unwrap_or((100.0, 40.0));

            elements.push(Box::new(SceneryElement::new(
                x,
                y,
                width,
                height,
                Color::WHITE,
                1, // Milieu
            )) as Box<dyn Drawable>);
        }
        SceneryType::Mountain => {
            let height = params.unwrap_or((200.0, 0.0)).0;

            elements.push(Box::new(SceneryElement::new(
                x,
                ground_level - height,
                300.0,
                height,
                Color::new(128, 128, 128, 255), // Gris
                0,                              // Arrière-plan
            )) as Box<dyn Drawable>);
        }
        SceneryType::Ground => {
            elements.push(Box::new(SceneryElement::new(
                x,
                ground_level,
                200.0,
                50.0,
                Color::new(76, 153, 0, 255), // Vert herbe
                2,                           // Premier plan
            )) as Box<dyn Drawable>);
        }
        SceneryType::GrassTuft => {
            elements.push(Box::new(SceneryElement::new(
                x,
                ground_level - 10.0,
                15.0,
                10.0,
                Color::new(102, 204, 0, 255), // Vert herbe plus vif
                2,                            // Premier plan
            )) as Box<dyn Drawable>);
        }
    }

    elements
}
