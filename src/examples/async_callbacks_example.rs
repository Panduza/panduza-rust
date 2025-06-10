/// Exemple d'utilisation des callbacks asynchrones
/// 
/// Ce fichier montre comment utiliser les nouvelles fonctionnalités de callbacks asynchrones
/// dans le système Panduza.

use panduza::{GenericAttribute, CallbackId};
use std::time::Duration;
use tokio::time::sleep;

// Type alias pour plus de lisibilité
type BooleanAttribute = GenericAttribute<panduza::fbs::BooleanBuffer>;

/// Exemple simple d'utilisation d'un callback asynchrone
pub async fn example_simple_async_callback() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration Zenoh (dans un vrai projet, ceci serait configuré différemment)
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await?;
      // Métadonnées de l'attribut
    let metadata = panduza::AttributeMetadata {
        topic: "device/relay/state".to_string(),
        r#type: "boolean".to_string(),
        info: None,
        mode: panduza::AttributeMode::ReadWrite,    };

    // Créer un attribut async
    let attribute: BooleanAttribute = GenericAttribute::new(session, metadata).await;

    // Ajouter un callback asynchrone simple
    let callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'un traitement asynchrone (par exemple, écriture en base de données)
                sleep(Duration::from_millis(100)).await;
                
                println!("✅ Callback asynchrone déclenché ! Buffer: {:?}", buffer);
                
                // Ici on peut faire des opérations asynchrones comme :
                // - Sauvegarder en base de données
                // - Envoyer une notification
                // - Appeler une API externe
                // - etc.
            })
        },
        None, // Pas de condition, déclenche toujours
    ).await;

    println!("Callback enregistré avec l'ID: {}", callback_id);
    
    // Simuler quelques changements de valeur
    // attribute.shoot(true).await; // Ceci déclencherait le callback
    
    Ok(())
}

/// Exemple avec condition et traitement plus complexe
pub async fn example_conditional_async_callback() -> Result<(), Box<dyn std::error::Error>> {
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await?;
      let metadata = panduza::AttributeMetadata {
        topic: "sensor/temperature/value".to_string(),
        r#type: "number".to_string(),
        info: None,
        mode: panduza::AttributeMode::ReadOnly,    };

    let attribute: GenericAttribute<panduza::fbs::NumberBuffer> = 
        GenericAttribute::new(session, metadata).await;

    // Callback avec condition : ne se déclenche que pour des valeurs élevées
    let alert_callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'un système d'alerte asynchrone
                sleep(Duration::from_millis(50)).await;
                
                println!("🚨 ALERTE TEMPÉRATURE ÉLEVÉE !");
                
                // Ici on pourrait :
                // - Envoyer un email d'alerte
                // - Déclencher un webhook
                // - Enregistrer l'incident
                send_temperature_alert().await;
            })
        },
        Some(|buffer| {
            // Condition : déclencher seulement si la température > 80°C
            // Note: cette logique dépendrait de la structure réelle du NumberBuffer
            // buffer.value() > 80.0  // Exemple conceptuel
            true // Pour la démo, on suppose toujours true
        }),
    ).await;

    // Callback de logging sans condition
    let log_callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'un logging asynchrone
                log_temperature_value(buffer).await;
            })
        },
        None,
    ).await;

    println!("Callbacks enregistrés - Alerte: {}, Log: {}", alert_callback_id, log_callback_id);
    
    Ok(())
}

/// Exemple d'attente d'une valeur spécifique avec timeout
pub async fn example_wait_for_value() -> Result<(), Box<dyn std::error::Error>> {
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await?;
      let metadata = panduza::AttributeMetadata {
        topic: "device/sensor/ready".to_string(),
        r#type: "boolean".to_string(),
        info: None,
    mode: panduza::AttributeMode::ReadOnly,
    };

    let attribute: BooleanAttribute = GenericAttribute::new(session, metadata).await;

    // Attendre que le capteur soit prêt (valeur true) avec un timeout de 10 secondes
    match attribute.wait_value(
        |buffer| {
            // Condition : attendre que la valeur soit true
            // buffer.value() == true  // Exemple conceptuel
            true // Pour la démo
        },
        Some(Duration::from_secs(10)),
    ).await {
        Ok(buffer) => {
            println!("✅ Capteur prêt ! Buffer reçu: {:?}", buffer);
        },
        Err(e) => {
            println!("❌ Timeout ou erreur en attendant que le capteur soit prêt: {}", e);
        }
    }
    
    Ok(())
}

/// Exemple de gestion avancée des callbacks
pub async fn example_callback_management() -> Result<(), Box<dyn std::error::Error>> {
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await?;
      let metadata = panduza::AttributeMetadata {
        topic: "device/status".to_string(),
        r#type: "string".to_string(),
        info: None,
        mode: panduza::AttributeMode::ReadWrite,
    };    let attribute: GenericAttribute<panduza::fbs::StringBuffer> = 
        GenericAttribute::new(session, metadata).await;

    // Enregistrer plusieurs callbacks
    let mut callback_ids = Vec::new();
    
    for i in 0..3 {
        let id = attribute.add_async_callback(
            move |buffer| {
                Box::pin(async move {
                    sleep(Duration::from_millis(i * 10)).await;
                    println!("Callback {} déclenché: {:?}", i, buffer);
                })
            },
            None,
        ).await;
        callback_ids.push(id);
    }

    println!("Nombre de callbacks enregistrés: {}", attribute.async_callback_count().await);

    // Supprimer un callback spécifique
    if let Some(&first_id) = callback_ids.first() {
        let removed = attribute.remove_async_callback(first_id).await;
        println!("Callback {} supprimé: {}", first_id, removed);
    }

    println!("Nombre de callbacks après suppression: {}", attribute.async_callback_count().await);

    // Nettoyer tous les callbacks
    attribute.clear_async_callbacks().await;
    println!("Tous les callbacks supprimés. Nombre restant: {}", attribute.async_callback_count().await);
    
    Ok(())
}

// Fonctions utilitaires pour les exemples

async fn send_temperature_alert() {
    // Simulation d'envoi d'alerte
    sleep(Duration::from_millis(200)).await;
    println!("📧 Email d'alerte envoyé !");
}

async fn log_temperature_value(buffer: &panduza::fbs::NumberBuffer) {
    // Simulation de logging
    sleep(Duration::from_millis(10)).await;
    println!("📊 Température enregistrée: {:?}", buffer);
}

/// Fonction principale pour tester tous les exemples
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Démarrage des exemples de callbacks asynchrones...\n");

    println!("1️⃣ Exemple simple de callback asynchrone:");
    example_simple_async_callback().await?;
    
    println!("\n2️⃣ Exemple avec condition:");
    example_conditional_async_callback().await?;
    
    println!("\n3️⃣ Exemple d'attente de valeur:");
    example_wait_for_value().await?;
    
    println!("\n4️⃣ Exemple de gestion des callbacks:");
    example_callback_management().await?;
    
    println!("\n✅ Tous les exemples terminés !");
    
    Ok(())
}
