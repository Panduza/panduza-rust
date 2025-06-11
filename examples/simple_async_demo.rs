/// Exemple simple d'utilisation des callbacks asynchrones
///
/// Ce programme démontre comment utiliser les nouveaux callbacks asynchrones
/// de Panduza pour traiter les événements de manière non-bloquante.

use panduza::{AsyncGenericAttribute, AttributeMetadata, AttributeMode};
use std::time::Duration;
use tokio::time::sleep;

// Type alias pour simplifier - utilisons BooleanBuffer qui implémente GenericBuffer
type AsyncBooleanAttribute = AsyncGenericAttribute<panduza::fbs::BooleanBuffer>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Démonstration des callbacks asynchrones Panduza\n");

    // Configuration Zenoh
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await.map_err(|e| format!("Erreur Zenoh: {}", e))?;

    // Métadonnées de l'attribut
    let metadata = AttributeMetadata {
        topic: "demo/relay_state".to_string(),
        r#type: "boolean".to_string(),
        info: Some("Démonstration des callbacks async".to_string()),
        mode: AttributeMode::ReadWrite,
    };

    // Créer un attribut asynchrone
    let attribute: AsyncBooleanAttribute = AsyncGenericAttribute::new(session, metadata).await;    println!("📡 Attribut créé sur le topic: {}", attribute.metadata().topic);

    // Exemple 1: Callback simple sans condition
    println!("\n1️⃣ Ajout d'un callback de logging...");
    let logging_callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'une opération async (ex: écriture en base)
                sleep(Duration::from_millis(50)).await;
                println!("📝 [LOG] État du relais: {:?}", buffer);
            })
        },
        None, // Pas de condition - déclenche toujours
    ).await;

    // Exemple 2: Callback avec condition pour détecter activation
    println!("2️⃣ Ajout d'un callback d'alerte pour activation...");
    let alert_callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'un système d'alerte
                sleep(Duration::from_millis(100)).await;
                println!("🚨 [ALERTE] Relais activé: {:?}", buffer);
                
                // Ici on pourrait faire des actions comme:
                // - Envoyer une notification
                // - Déclencher d'autres systèmes
                // - Logger en priorité élevée
            })
        },
        Some(|buffer| {
            // Condition: déclencher seulement quand le relais est activé (true)
            // Pour l'instant on simule avec un debug print check
            let buffer_str = format!("{:?}", buffer);
            buffer_str.contains("true") // Approximation pour la démo
        }),
    ).await;

    // Exemple 3: Callback de métriques
    println!("3️⃣ Ajout d'un callback de métriques...");
    let _metrics_callback_id = attribute.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'envoi de métriques
                sleep(Duration::from_millis(25)).await;
                println!("📊 [METRICS] Mise à jour des statistiques pour: {:?}", buffer);
            })
        },
        None,
    ).await;

    println!("\n✅ {} callbacks enregistrés", attribute.async_callback_count().await);

    // Simulation de quelques événements
    println!("\n📤 Simulation d'événements...");
    
    // Ceci déclenche le logging et les métriques
    // attribute.shoot("INFO: System started".to_string()).await;
    
    // Petit délai pour voir les callbacks s'exécuter
    sleep(Duration::from_millis(200)).await;
    
    // Ceci déclenche tous les callbacks (y compris l'alerte)
    // attribute.shoot("ERROR: Critical failure detected".to_string()).await;
    
    sleep(Duration::from_millis(200)).await;

    // Exemple 4: Attente d'une valeur spécifique
    println!("\n4️⃣ Test d'attente d'une valeur spécifique...");
    
    // Dans un vrai scénario, on pourrait attendre une confirmation
    // let result = attribute.wait_value(
    //     |buffer| format!("{:?}", buffer).contains("READY"),
    //     Some(Duration::from_secs(5))
    // ).await;
    
    // match result {
    //     Ok(buffer) => println!("✅ Valeur attendue reçue: {:?}", buffer),
    //     Err(e) => println!("❌ Erreur en attente: {}", e),
    // }

    // Gestion des callbacks
    println!("\n🧹 Nettoyage des callbacks...");
    
    // Supprimer un callback spécifique
    let removed = attribute.remove_async_callback(logging_callback_id).await;
    println!("Callback de logging supprimé: {}", removed);
    
    // Supprimer un autre callback
    let removed = attribute.remove_async_callback(alert_callback_id).await;
    println!("Callback d'alerte supprimé: {}", removed);
    
    println!("Callbacks restants: {}", attribute.async_callback_count().await);
    
    // Nettoyer tous les callbacks
    attribute.clear_async_callbacks().await;
    println!("Tous les callbacks supprimés. Total: {}", attribute.async_callback_count().await);

    println!("\n🎉 Démonstration terminée avec succès !");
    
    Ok(())
}
