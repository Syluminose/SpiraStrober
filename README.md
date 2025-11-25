# 🌈 SpiraStrober

**Lien vers l'application :**
[https://spirastrober.netlify.app/](https://spirastrober.netlify.app/)

---

## 🎯 But de l'Application

SpiraStrober est un outil de création de **séquences lumineuses** pour les systèmes de stimulation audiovisuelle.

Son rôle est de générer un **fichier audio numérique (.wav)** contenant les informations d'éclairage codées.

Le **Format de Codage** est Spectrastrobe et Audiostrobe.
La **Nature du Signal** est un signal ultrasonore de haute fréquence (non audible par l'oreille humaine).
**Aucun son audible n'est généré** par l'application.

---

## ⚙️ Principes Fondamentaux et Paramètres

La lumière de stimulation est un signal modulé mathématiquement. Chaque séquence (segment de temps) définit une transition progressive entre deux ensembles de paramètres.

### 1. Le Signal Lumineux (Fréquence et Phase)

Le signal lumineux de chaque couleur (Rouge, Vert, Bleu) est une **onde sinusoïdale** dont la fréquence de clignotement (lumière) et la phase évoluent au fil du temps.

* **⏳ Durée (s) :** La durée en secondes de la séquence active. C'est la vitesse à laquelle la transition vers les paramètres suivants s'effectue.
* **🔔 Fréquence (Hz) :** La fréquence de clignotement de la lumière en Hertz (cycles par seconde). Elle peut varier linéairement (rampe) entre le début et la fin de la séquence.
* **⚡ Phase (tours) :** Le décalage initial de l'onde lumineuse, exprimé en tours (1 tour = $2\pi$ radians). Elle détermine le point de départ du clignotement.

### 2. Modulation de Fréquence (Chirp)

Le terme **chirp** (dérivée de la fréquence) est utilisé pour décrire comment la fréquence elle-même change au cours du temps. Il s'agit de la **vitesse d'accélération ou de décélération** de la fréquence lumineuse.

* **🌊 OscAmp (Hz) :** L'amplitude (en Hz) de la modulation de fréquence appliquée au signal. C'est l'intensité de la modulation.
* **💫 OscFreq (Hz) :** La fréquence de cette modulation. Si cette valeur est non nulle, la fréquence de clignotement elle-même **oscille** autour de sa valeur principale (🔔).
* **✨ PhaseOsc (tours) :** Le décalage initial (phase) de l'onde d'oscillation.

---

## 🎨 Visualisation et Encodage

* **Visualisation :** L'écran de visualisation **"🌈SpiraStrober"** affiche les trois courbes mathématiques calculées à partir de vos paramètres :
    * **Signal (Sᵢ) :** La courbe sinusoïdale de la lumière (l'onde de clignotement).
    * **Fréquence (fᵢ) :** La fréquence instantanée réelle (🔔 + l'effet d'oscillation).
    * **Chirp (kᵢ) :** Le taux de changement de la fréquence au fil du temps (l'accélération ou la décélération).

* **Encodage Ultrasonore :** Le signal final (Sᵢ) est combiné avec une porteuse ultrasonore (autour de 18-20 kHz) pour créer le fichier audio. C'est ce fichier non audible qui est lu par les appareils Spectrastrobe/Audiostrobe pour recréer la lumière.

---

## 💡 Prise en Main : Génération du Fichier

1. **Configuration des Séquences :** Utilisez le panneau des paramètres pour ajuster les valeurs des ondes lumineuses pour chaque séquence. Les boutons d'action (🔶, 🔁, ☀️, etc.) dans la barre supérieure et à côté de chaque séquence permettent de créer des variations ou de copier des configurations.

2. **Choix du Mode d'Encodage :** En bas de l'écran, utilisez la liste déroulante pour choisir le mode d'encodage du signal ultrasonore :
Spectra (🌈Spectra) : Encode les trois canaux (Rouge, Vert, Bleu) dans le signal ultrasonore.
Rouge/Vert/Bleu (🔴, 🟢, 🔵) : Encode un seul canal pour l'adapter aux systèmes plus anciens.
Unifié (☀️Unifié) : Encode un signal combiné (par défaut : Vert) sur les deux canaux stéréo.

3. **Génération :** Appuyez sur le bouton **"🎵 Générer"**. L'application calcule et génère le fichier audio.

4. **Exportation :** Une fois la génération terminée, le fichier audio (.wav) est prêt à être téléchargé. Ce fichier contient l'encodage ultrasonore pour piloter votre appareil Spectrastrobe/Audiostrobe.
