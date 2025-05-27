use panduza::reactor::{new_reactor, ReactorOptions};

#[tokio::main]
async fn main() {
    println!("Running free run test");

    // Créer les options pour se connecter à localhost sur le port MQTT par défaut (1883)
    let options = ReactorOptions::new("localhost", 1883);

    // Créer et initialiser le réacteur
    match new_reactor(options).await {
        Ok(reactor) => {
            println!("Reactor créé avec succès, connexion à localhost:1883");

            // Obtenir l'attribut de notification
            let notification_attr = reactor.new_notification_attribute().await;
            println!("Attribut de notification créé avec succès");

            // Ajouter un callback pour traiter les notifications reçues
            notification_attr.add_callback(|notification| {
                Box::pin(async move {
                    println!("Notification reçue:");
                    println!("  - Source: {:?}", notification.object().source());
                    println!("  - Message: {:?}", notification.object().message());
                    println!("  - Type: {:?}", notification.object().type_());
                    println!("  - Timestamp: {:?}", notification.object().timestamp());
                })
            });

            println!("Callback ajouté, en attente de notifications...");

            // Attendre indéfiniment pour recevoir des notifications
            // En production, vous pourriez vouloir ajouter une condition d'arrêt
            loop {
                notification_attr.update_notifier().notified().await;
                println!("Nouvelle notification reçue!");
            }
        }
        Err(e) => {
            eprintln!("Erreur lors de la création du reactor: {}", e);
            std::process::exit(1);
        }
    }
}
