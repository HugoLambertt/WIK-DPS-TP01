use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;
use std::time::Instant;
use std::sync::{Arc, Mutex};

// L'interface (Trait) demandée par la consigne
trait CounterStore {
    fn increment(&self);
    fn get_count(&self) -> u32;
}

// L'implémentation concrète en mémoire
struct MemoryCounterStore {
    // Mutex permet de modifier le compteur en toute sécurité entre plusieurs threads
    count: Mutex<u32>,
}

impl MemoryCounterStore {
    fn new() -> Self {
        Self {
            count: Mutex::new(0),
        }
    }
}

impl CounterStore for MemoryCounterStore {
    fn increment(&self) {
        let mut num = self.count.lock().unwrap();
        *num += 1;
    }

    fn get_count(&self) -> u32 {
        let num = self.count.lock().unwrap();
        *num
    }
}


fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let adresse = format!("0.0.0.0:{}", port);

    // Initialisation de l'uptime et du stockage du compteur
    let start_time = Instant::now();
    let counter_store = Arc::new(MemoryCounterStore::new());

    let ecouteur = TcpListener::bind(&adresse).expect("Impossible de lier le port");
    println!("Serveur en écoute sur http://{}", adresse);

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();
        
        // On clone les pointeurs Arc pour les passer à la fonction (légère copie de pointeur)
        let counter_clone = Arc::clone(&counter_store);
        
        // On passe l'uptime et le compteur à notre gestionnaire
        gestion_connexion(flux, counter_clone, start_time);
    }
}

fn gestion_connexion(mut flux: TcpStream, counter: Arc<MemoryCounterStore>, start_time: Instant) {
    let mut tampon = [0; 4096];
    let octets_lus = flux.read(&mut tampon).unwrap();
    if octets_lus == 0 { return; }

    let requete_brute = String::from_utf8_lossy(&tampon[..octets_lus]);
    let mut lignes = requete_brute.lines();
    
    let premiere_ligne = lignes.next().unwrap_or("");
    let parties: Vec<&str> = premiere_ligne.split_whitespace().collect();
    
    if parties.len() >= 2 && parties[0] == "GET" {
        match parties[1] {
            // --- ROUTE PING ---
            "/ping" => {
                // Consigne : Le compteur s'incrémente à chaque requête /ping comprise
                counter.increment();

                let mut headers = HashMap::new();
                for line in lignes {
                    if line.is_empty() { break; }
                    if let Some((cle, valeur)) = line.split_once(':') {
                        headers.insert(cle.trim().to_string(), valeur.trim().to_string());
                    }
                }

                let reponse_json = serde_json::to_string(&headers).unwrap_or_else(|_| "{}".to_string());
                envoyer_reponse_json(flux, "200 OK", reponse_json);
            },

            // --- ROUTE STATS (BONUS) ---
            "/stats" => {
                let uptime = start_time.elapsed().as_secs();
                let instance_id = env::var("INSTANCE_ID").unwrap_or_else(|_| "instance-locale-default".to_string());
                let total_requests = counter.get_count();

                // Construction de l'objet JSON pour les stats
                let stats_json = serde_json::json!({
                    "total_requests": total_requests,
                    "uptime_seconds": uptime,
                    "instance_id": instance_id
                }).to_string();

                envoyer_reponse_json(flux, "200 OK", stats_json);
            },

            // --- TOUT LE RESTE -> 404 ---
            _ => {
                let reponse_404 = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                let _ = flux.write_all(reponse_404.as_bytes());
                let _ = flux.flush();
            }
        }
    }
}

// Fonction utilitaire pour éviter la duplication de code pour l'envoi de JSON
fn envoyer_reponse_json(mut flux: TcpStream, statut: &str, corps_json: String) {
    let reponse = format!(
        "HTTP/1.1 {}\r\n\
        Content-Type: application/json\r\n\
        Content-Length: {}\r\n\
        Connection: close\r\n\r\n\
        {}",
        statut,
        corps_json.len(),
        corps_json
    );
    let _ = flux.write_all(reponse.as_bytes());
    let _ = flux.flush();
}