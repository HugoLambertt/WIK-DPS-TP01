use std::env;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

fn main() {
    let port = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let adresse = format!("0.0.0.0:{}", port);

    let ecouteur = TcpListener::bind(&adresse).expect("Impossible de lier le port");
    println!("Serveur en écoute sur http://{}", adresse);

    for flux in ecouteur.incoming() {
        let flux = flux.unwrap();
        gestion_connexion(flux);
    }
}

fn gestion_connexion(mut flux: TcpStream) {
    let mut tampon = [0; 4096];
    let octets_lus = flux.read(&mut tampon).unwrap();
    if octets_lus == 0 { return; }

    let requete_brute = String::from_utf8_lossy(&tampon[..octets_lus]);
    let mut lignes = requete_brute.lines();
    
    let premiere_ligne = lignes.next().unwrap_or("");
    let parties: Vec<&str> = premiere_ligne.split_whitespace().collect();
    
    if parties.len() >= 2 && parties[0] == "GET" && parties[1] == "/ping" {
        let mut headers = HashMap::new();
        
        for ligne in lignes {
            if ligne.is_empty() { break; }
            if let Some((cle, valeur)) = ligne.split_once(':') {
                headers.insert(cle.trim().to_string(), valeur.trim().to_string());
            }
        }

        let reponse_json = serde_json::to_string(&headers).unwrap_or_else(|_| "{}".to_string());
        
        let reponse = format!(
            "HTTP/1.1 200 OK\r\n\
            Content-Type: application/json\r\n\
            Content-Length: {}\r\n\
            Connection: close\r\n\r\n\
            {}",
            reponse_json.len(),
            reponse_json
        );
        
        flux.write_all(reponse.as_bytes()).unwrap();
        flux.flush().unwrap();
        } else {
                let reponse_404 = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
                flux.write_all(reponse_404.as_bytes()).unwrap();
                
                flux.flush().unwrap(); 
    }
}