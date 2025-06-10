# Callbacks Asynchrones Panduza

Cette fonctionnalité ajoute le support des callbacks asynchrones à la bibliothèque Panduza, permettant un traitement non-bloquant des événements.

## 🆕 Nouvelles Fonctionnalités

### Types Ajoutés

#### `AsyncCallbackFn<T>`
Type alias pour les fonctions de callback asynchrones:
```rust
pub type AsyncCallbackFn<T> = Box<dyn Fn(&T) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> + Send + Sync>;
```

#### `AsyncCallbackEntry<T>`
Structure contenant un callback asynchrone et une condition optionnelle:
```rust
pub struct AsyncCallbackEntry<T> {
    pub callback: AsyncCallbackFn<T>,
    pub condition: Option<ConditionFn<T>>,
}
```

#### `GenericAttribute<B>`
Implémentation d'attribut générique avec support des callbacks asynchrones.

### Fonctionnalités Principales

1. **Callbacks Asynchrones**: Exécution non-bloquante des callbacks
2. **Conditions Optionnelles**: Filtrage des événements déclencheurs
3. **Exécution Concurrente**: Tous les callbacks sont exécutés en parallèle
4. **Gestion des Callbacks**: Ajout, suppression et nettoyage des callbacks
5. **Attente de Valeur**: Possibilité d'attendre une valeur spécifique avec timeout

## 📖 Guide d'Utilisation

### Création d'un Attribut Asynchrone

```rust
use panduza::{GenericAttribute, AttributeMetadata, AttributeMode};

// Configuration
let config = zenoh::config::Config::default();
let session = zenoh::open(config).await?;

let metadata = AttributeMetadata {
    topic: "device/status".to_string(),
    r#type: "string".to_string(),
    info: None,
    mode: AttributeMode::ReadWrite,
};

// Créer l'attribut
let attribute: GenericAttribute<StringBuffer> = 
    GenericAttribute::new(session, metadata).await;
```

### Ajout de Callbacks Asynchrones

#### Callback Simple
```rust
let callback_id = attribute.add_async_callback(
    |buffer| {
        Box::pin(async move {
            // Traitement asynchrone
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("Valeur reçue: {:?}", buffer);
            
            // Ici vous pouvez faire:
            // - Écriture en base de données
            // - Appels API
            // - Notifications
            // - etc.
        })
    },
    None, // Pas de condition
).await;
```

#### Callback avec Condition
```rust
let alert_callback_id = attribute.add_async_callback(
    |buffer| {
        Box::pin(async move {
            // Traitement d'alerte asynchrone
            send_alert_email().await;
            log_critical_event(buffer).await;
        })
    },
    Some(|buffer| {
        // Condition: déclencher seulement pour les erreurs
        buffer.contains_error()
    }),
).await;
```

### Attente de Valeur Spécifique

```rust
// Attendre une valeur avec timeout
let result = attribute.wait_value(
    |buffer| buffer.is_ready(),
    Some(Duration::from_secs(10))
).await;

match result {
    Ok(buffer) => println!("Valeur attendue reçue: {:?}", buffer),
    Err(e) => println!("Timeout ou erreur: {}", e),
}
```

### Gestion des Callbacks

```rust
// Compter les callbacks
let count = attribute.async_callback_count().await;

// Supprimer un callback spécifique
let removed = attribute.remove_async_callback(callback_id).await;

// Nettoyer tous les callbacks
attribute.clear_async_callbacks().await;
```

## 🔍 Exemples

### Exemple Complet: Système de Monitoring

```rust
use panduza::{AsyncGenericAttribute, AttributeMetadata, AttributeMode};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = zenoh::config::Config::default();
    let session = zenoh::open(config).await?;
    
    let metadata = AttributeMetadata {
        topic: "sensor/temperature".to_string(),
        r#type: "number".to_string(),
        info: None,
        mode: AttributeMode::ReadOnly,
    };

    let sensor: AsyncGenericAttribute<NumberBuffer> = 
        AsyncGenericAttribute::new(session, metadata).await;

    // Callback de logging général
    let _log_id = sensor.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'écriture en base
                sleep(Duration::from_millis(50)).await;
                println!("📊 Température enregistrée: {:?}", buffer);
            })
        },
        None,
    ).await;

    // Callback d'alerte haute température
    let _alert_id = sensor.add_async_callback(
        |buffer| {
            Box::pin(async move {
                // Simulation d'envoi d'alerte
                sleep(Duration::from_millis(100)).await;
                println!("🚨 ALERTE: Température critique!");
                send_sms_alert().await;
                trigger_cooling_system().await;
            })
        },
        Some(|buffer| {
            // Condition: température > 80°C
            extract_temperature(buffer) > 80.0
        }),
    ).await;

    // Le système est maintenant prêt à traiter les événements
    println!("✅ Système de monitoring actif");
    
    Ok(())
}

async fn send_sms_alert() {
    // Simulation d'envoi SMS
    sleep(Duration::from_millis(200)).await;
    println!("📱 SMS d'alerte envoyé");
}

async fn trigger_cooling_system() {
    // Simulation d'activation du système de refroidissement
    sleep(Duration::from_millis(300)).await;
    println!("❄️ Système de refroidissement activé");
}

fn extract_temperature(buffer: &NumberBuffer) -> f64 {
    // Extraction de la température depuis le buffer
    // (implémentation dépendante de la structure réelle)
    25.0 // Valeur d'exemple
}
```

## 🚀 Avantages des Callbacks Asynchrones

1. **Performance**: Traitement non-bloquant des événements
2. **Concurrence**: Exécution parallèle de plusieurs callbacks
3. **Flexibilité**: Conditions personnalisables pour le déclenchement
4. **Intégration**: Compatible avec l'écosystème async/await de Rust
5. **Robustesse**: Gestion d'erreur et timeout intégrés

## 🆚 Comparaison avec les Callbacks Synchrones

| Fonctionnalité | Callbacks Sync | Callbacks Async |
|----------------|----------------|-----------------|
| Exécution | Bloquante | Non-bloquante |
| Concurrence | Séquentielle | Parallèle |
| I/O | Bloque le thread | Asynchrone |
| Performance | Limitée | Optimale |
| Complexité | Simple | Légèrement plus complexe |

## 📝 Notes Techniques

- Les callbacks asynchrones utilisent `tokio::sync::Mutex` pour la synchronisation
- L'exécution parallèle est gérée avec `futures::future::join_all`
- Les conditions de filtrage restent synchrones pour des performances optimales
- La méthode `wait_value` utilise un `broadcast` channel pour éviter les problèmes de ownership

## 🔮 Utilisation Future

Cette fonctionnalité ouvre la voie à des patterns avancés comme:
- Systèmes de monitoring distribués
- Pipelines de traitement de données asynchrones
- Intégration avec des services externes
- Gestion d'état réactif
- Systèmes d'événements complexes

## 🐛 Résolution de Problèmes

### Erreur de Compilation avec `move`
Si vous rencontrez des erreurs liées au `move` dans les closures, assurez-vous d'utiliser `clone()` pour les valeurs que vous voulez déplacer.

### Performance
Les callbacks asynchrones sont plus appropriés pour les opérations I/O. Pour les traitements CPU intensifs, préférez les callbacks synchrones.

### Gestion de la Mémoire
N'oubliez pas de nettoyer les callbacks inutilisés pour éviter les fuites mémoire.
