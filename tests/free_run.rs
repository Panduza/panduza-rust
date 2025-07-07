use panduza::reactor::{new_reactor, ReactorOptions};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    println!("Running free run test");

    // Créer les options pour se connecter à localhost sur le port MQTT par défaut (1883)
    let options = ReactorOptions::new("127.0.0.1", 7447, "minica.pem", None);

    // Créer et initialiser le réacteur
    match new_reactor(options).await {
        Ok(reactor) => {
            println!("Reactor créé avec succès");
            let shared_counter = Arc::new(Mutex::new(0i32));

            // Obtenir l'attribut de notification
            let notification_attr = reactor.new_notification_attribute().await;
            println!("Attribut de notification créé avec succès");

            // // Premier callback - incrémente le compteur et affiche les détails
            // let counter_ref1 = Arc::clone(&shared_counter);
            // notification_attr.add_callback(move |notification| {
            //     let counter = Arc::clone(&counter_ref1);
            //     Box::pin(async move {
            //         let mut count = counter.lock().unwrap();
            //         *count += 1;
            //         let current_count = *count;
            //         drop(count); // Libérer le lock

            //         println!("=== CALLBACK 1 ===");
            //         println!("Compteur: {}", current_count);
            //         println!("Notification reçue:");
            //         println!("  - Source: {:?}", notification.object().source());
            //         println!("  - Message: {:?}", notification.object().message());
            //         println!("  - Type: {:?}", notification.object().type_());
            //         println!("  - Timestamp: {:?}", notification.object().timestamp());
            //     })
            // });

            // // Deuxième callback - incrémente aussi le compteur et affiche un résumé
            // let counter_ref2 = Arc::clone(&shared_counter);
            // notification_attr.add_callback(move |notification| {
            //     let counter = Arc::clone(&counter_ref2);
            //     Box::pin(async move {
            //         let mut count = counter.lock().unwrap();
            //         *count += 1;
            //         let current_count = *count;
            //         drop(count); // Libérer le lock

            //         println!("=== CALLBACK 2 ===");
            //         println!("Compteur: {}", current_count);
            //         println!(
            //             "Résumé: Notification de type {:?} reçue",
            //             notification.object().type_()
            //         );
            //     })
            // });
            // println!("Deux callbacks ajoutés, en attente de notifications...");

            // Attendre indéfiniment pour recevoir des notifications
            // En production, vous pourriez vouloir ajouter une condition d'arrêt
            // loop {
            //     notification_attr.update_notifier().notified().await;

            //     // Afficher le compteur total après chaque cycle de notifications
            //     let total_count = shared_counter.lock().unwrap();
            //     println!(">>> Total notifications traitées: {} <<<", *total_count);
            // }
        }
        Err(e) => {
            eprintln!("Erreur lors de la création du reactor: {}", e);
            std::process::exit(1);
        }
    }
}
