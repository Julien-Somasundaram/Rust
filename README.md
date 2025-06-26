# Rust
# Groupe
- Somasundaram Julien
- Tamimoul Chuaibe
- Choura Hakim
- Sasirajah Sasthiga
- BenElHadj Brahim

# 🌌 EREEA - Essaim de Robots pour l'Exploration et l'Étude Astrobiologique

> Simulation en Rust d’un essaim de robots autonomes coopérant pour explorer une carte générée procéduralement, collecter des ressources et transmettre les informations à une station de base.

---

## 🧠 Sommaire

- [🎯 Objectif du projet](#-objectif-du-projet)
- [🚀 Fonctionnalités](#-fonctionnalités)
- [🗂️ Architecture du code](#️-architecture-du-code)
- [🛠️ Installation](#️-installation)
- [▶️ Utilisation](#️-utilisation)
- [🧪 Tests](#-tests)
- [📷 Captures (optionnel)](#-captures-optionnel)
- [📚 Ressources](#-ressources)
- [📜 Licence](#-licence)
- [🤝 Contribuer](#-contribuer)

---

## 🎯 Objectif du projet

Le projet **EREEA** vise à modéliser et simuler le comportement d’un essaim de robots pour des missions spatiales astrobiologiques :

- Déploiement sur une carte hostile et inconnue
- Exploration intelligente et modulaire
- Collecte différée d’informations par retour à la base
- Gestion de la mémoire, des threads et de la concurrence via Rust

---

## 🚀 Fonctionnalités

- 🧭 **Carte 2D procédurale** (bruit de gradient) avec seed configurable
- 🧱 Obstacles, bordures, topologie plane ou sphérique (bonus)
- ⚡ **Ressources localisées** : Énergie, Minerais, Sites scientifiques
- 🤖 **Robots spécialisés** (exploration, collecte, transmission, etc.)
- 🧠 **Comportement autonome** avec intelligence locale et mémoire partagée
- 🛰️ **Station de base** collectant et consolidant les découvertes
- 🔁 **Partage d'information différé** (modèle de synchronisation git-like)
- 🎛️ **Interface terminal (TUI)** avec `ratatui` (optionnel)
- 🔬 **Tests unitaires et d’intégration**
- ⏱️ **Gestion de la concurrence** (RwLock, Arc, Channels, etc.)

---

### lancement
```bash
cargo run
```
### Test
```bash
cargo test
```
