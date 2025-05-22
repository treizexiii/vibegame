# Vibe Game

Un jeu de plateforme 2D minimaliste développé en Rust avec la bibliothèque Raylib.

## Description

Vibe Game est un jeu de plateforme simple où vous contrôlez un personnage qui peut se déplacer horizontalement et sauter. Le jeu se distingue par son système de saut avec charge, où maintenir la touche d'espace permet de charger un saut plus puissant.

## Fonctionnalités

- Graphismes 2D simples mais expressifs
- Défilement horizontal avec parallaxe
- Système de saut avec charge de puissance
- Effet d'ombre dynamique qui réagit à la hauteur du saut
- Monde généré procéduralement avec divers éléments de décor

## Contrôles

- **Flèches Gauche/Droite**: Déplacer le personnage
- **Barre d'espace** (maintenir): Charger un saut
- **Barre d'espace** (relâcher): Effectuer le saut

## Prérequis

- Rust (édition 2021 ou plus récente)
- Dépendances :
  - raylib (v3.7+)
  - rand (v0.8+)

## Installation

1. Clonez ce dépôt :
   ```
   git clone https://github.com/treizexiii/vibegame.git
   ```
2. Accédez au répertoire du projet :
   ```
   cd vibegame
   ```
3. Installez les dépendances requises :
   ```
   cargo build
   ```
4. Exécutez le jeu :
   ```
   cargo run
   ```

## Architecture du code

Le projet suit une architecture modulaire avec plusieurs composants :

- `main.rs` : Point d'entrée et boucle principale du jeu
- `player.rs` : Logique et rendu du personnage jouable
- `scenery.rs` : Gestion des éléments du décor et du monde
- `drawable.rs` : Trait commun pour les objets affichables
- `utils.rs` : Fonctions utilitaires (génération du monde, gestion des entrées)

## Développement futur

- Ajout d'obstacles et d'ennemis
- Système de collectibles
- Niveaux avec objectifs spécifiques
- Effets sonores et musique d'ambiance
- Menu principal et système de score

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à proposer une pull request.

## Licence
Ce projet est sous licence MIT. Voir le fichier LICENSE pour plus de détails.

## Remerciements

- [Raylib](https://www.raylib.com/) pour la bibliothèque graphique
- [Rust](https://www.rust-lang.org/) pour le langage de programmation