use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use serde_json::Value as JsonValue;

use crate::{asyncv::reactor::DataReceiver, attribute_metadata::AttributeMetadata};

#[derive(Debug)]
struct StructureData {
    ///
    ///
    brut: JsonValue,

    /// Structure extracted and prepared for easy access
    ///
    flat: HashMap<String, AttributeMetadata>,
}

impl StructureData {
    pub fn new() -> StructureData {
        Self {
            brut: JsonValue::Null,
            flat: HashMap::new(),
        }
    }

    ///
    ///
    pub fn update(&mut self, payload: Bytes) -> Result<(), String> {
        //
        self.brut = serde_json::from_slice(&payload).map_err(|e| e.to_string())?;

        //
        if let Some(driver_instances) = self.brut.get("driver_instances") {
            if let Some(driver_instances) = driver_instances.as_object() {
                //
                for (instance_names, body) in driver_instances.iter() {
                    println!("Key: {}, Value: {}", instance_names, body);
                }
            }
        }

        Ok(())
    }

    ///
    ///
    pub fn register_flat_entry(&mut self, name: String, data: JsonValue) {}
}

#[derive(Clone, Debug)]
/// Object to manage the structure
///
pub struct Structure {
    /// initial data
    ///
    value: Arc<Mutex<StructureData>>,
    // pza_structure_flat
}

impl Structure {
    pub fn new(mut data_receiver: DataReceiver) -> Self {
        let json_value = Arc::new(Mutex::new(StructureData::new()));

        let json_value_2 = json_value.clone();
        tokio::spawn(async move {
            loop {
                let message = data_receiver.recv().await;
                println!("!!!!!!!!!!! ssss ttt Notification = {:?}", message);

                if let Some(message) = message {
                    match json_value_2.lock() {
                        Ok(mut deref_value) => {
                            deref_value.update(message);
                        }
                        Err(e) => {
                            println!("Error = {:?}", e);
                        }
                    }
                }
            }
        });

        Structure { value: json_value }
    }

    // pub fn update(&mut self, json_value: JsonValue) {
    //     self.json_value = json_value;
    // }

    // pub async fn run(&mut self) {

    // }
}
