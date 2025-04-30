<div>
<img alt="logo" src="./src-tauri/icons/icon_64.png"> 
<span 
    style="margin-left: 10px; font-size: 30px; opacity: 0.2; letter-spacing: 2px; text-shadow: 2px 1px 1px #4b1d8e">
FLOWS 
</span>
</div>

## 🚀 Introduction

_**Flows**_ est une _**menu bar app**_ pour macos.

Il permet d'enregistrer et de centraliser les actions habituellement executées telles que la _[navigation vers une page web](#navigations)_ ou l'_[execution d'une commande](#commands)_.  

## ⚙️ Configuration
L'application se construit sur la base de la _⚙️ configuration_ fournie.

Cette configuration est au format `json` et se trouve à l'emplacement suivant : `/$USER/.wfapp/config.json`.

Au premier lancement de l'application l'arborescence suivante :
``` 
.
├── 📂 .wfapp
│   ├── 📄 config.json
│   ├── 📂 icons
```
est créée :

Le dossier `📂 .wfapp` contient les fichiers et sous-dossiers de configuration

Le fichier `📄 config.json` est le fichier de configuration. La structure de configuration attendue est la suivante :  

```json
{
  "path" : "",
  "variables" : {},
  "secrets" : {},
  "navigations" : [
    {
      "name": "",
      "url": "",
      "icon" : ""
    }
  ],
  "commands": [
    {
      "name" : "",
      "cmd" : ""
    }
  ]
}
```

Le dossier `📂 icons` contient les icons utilisées pour les [navigations](#navigations).

<br>

Petit tour d'horizon sur les différents propriétés de la configuration : 

* ### <span id="path">🛤️ Path

C'est la valeur de votre variable d'environnement `$PATH`. Il est utilisé lors de l'exécution des commandes. 

En effet, **_Flows_** n'a pas accès à vos fichiers `.rc`, il n'a donc pas connaissance de votre `$PATH` et ne saurait donc pas retrouver vos différentes commandes. 

* ### <span id="variables">Variables

* ### <span id="secrets"> 🔐Secrets

* ### <span id="navigations">Navigations

* ### <span id="commands">Commands