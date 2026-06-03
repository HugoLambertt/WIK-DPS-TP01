# WIK-DPS-TP01


1/ Installation de Rust via la documentation officielle.

2/ Création de l'environnement du projet avec la commande :

3/ Ajout de la dépendance serde_json = "1.0" dans le fichier Cargo.toml pour formater les données en JSON.

4/ Test initial de l'écoute des flux TCP (Encart 20-1 du Rust Book) :

use std::net::TcpListener;

fn main() {
    let ecouteur = TcpListener::bind("127.0.0.1:7878").unwrap();
    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();
        println!("Connexion établie !");
    }
}

5/ Remplacement de l'IP et du port fixes par la variable d'environnement PING_LISTEN_PORT (avec le port 8080 par défaut si elle n'est pas définie) :

let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());


6/ Mise en place de l'écoute sur toutes les interfaces réseau avec l'adresse 0.0.0.0 combinée au port dynamique.

7/ Lecture de la requête HTTP brute et découpage ligne par ligne pour isoler la première ligne (Request Line) et les entêtes (Headers).

8/ Filtrage strict de la route :

- Si la requête est un GET sur /ping, le programme extrait les headers dans une HashMap, les convertit en JSON avec serde_json et renvoie un code 200 OK.

- Pour tout autre chemin (ex: /test), le serveur renvoie immédiatement une réponse vide avec un code 404 NOT FOUND et ferme la connexion avec flux.flush().


9/ Étape Bonus : Mode réplication & Statistiques
- Isolation du compteur derrière une interface (Trait `CounterStore`) et une implémentation en mémoire (`MemoryCounterStore`).
- Sécurisation des données pour le multi-threading avec un pointeur partagé `Arc` et un verrou `Mutex`.
- Ajout de la route `GET /stats` qui renvoie le compteur de pings, l'uptime et l'ID d'instance.

## Phase de Test Réels (PowerShell)

### Test 1 : Lancement initial et vérification des stats (0 requête)
```powershell
PS C:\www\WIK-DPS-TP01> Invoke-RestMethod -Uri "http://localhost:8080/stats"

instance_id             total_requests uptime_seconds
-----------             -------------- --------------
instance-locale-default              0             52


PS C:\www\WIK-DPS-TP01> Invoke-RestMethod -Uri "http://localhost:8080/ping" 

User-Agent                                                                        Host          
----------                                                                        ----          
Mozilla/5.0 (Windows NT; Windows NT 10.0; fr-FR) WindowsPowerShell/5.1.26100.8457 localhost:8080


Test 3 : Vérification de l'incrémentation du compteur
PowerShell
PS C:\www\WIK-DPS-TP01> Invoke-RestMethod -Uri "http://localhost:8080/stats"

instance_id             total_requests uptime_seconds
-----------             -------------- --------------
instance-locale-default              1             86


Test 4 : Simulation d'une instance répliquée avec un autre nom
PowerShell
PS C:\www\WIK-DPS-TP01\srv-web> $env:INSTANCE_ID="srv-web-cyber-prod-01"
PS C:\www\WIK-DPS-TP01\srv-web> cargo run

Serveur en écoute sur [http://0.0.0.0:8080](http://0.0.0.0:8080)

Vérification des stats sur la nouvelle instance (le compteur repart à 0 et le nom a changé) :

PowerShell
PS C:\www\WIK-DPS-TP01> Invoke-RestMethod -Uri "http://localhost:8080/stats"

instance_id           total_requests uptime_seconds
-----------           -------------- --------------
srv-web-cyber-prod-01              0             21