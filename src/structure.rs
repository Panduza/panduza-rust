use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{attribute_metadata::AttributeMetadata, reactor::DataReceiver};
use bytes::Bytes;
use serde_json::{Map, Value as JsonValue};
use yash_fnmatch::{without_escape, Pattern};

use tokio::sync::Notify;

#[derive(Debug)]
struct StructureData {
    ///
    ///
    brut: JsonValue,

    /// Structure extracted and prepared for easy access
    ///
    flat: HashMap<String, AttributeMetadata>,

    ///
    ///
    initialized: Arc<Notify>,
}

impl StructureData {
    ///
    ///
    pub fn new() -> StructureData {
        Self {
            brut: JsonValue::Null,
            flat: HashMap::new(),
            initialized: Arc::new(Notify::new()),
        }
    }

    ///
    ///
    pub fn update(&mut self, payload: Bytes) -> Result<(), String> {
        //
        let brut: JsonValue = serde_json::from_slice(&payload).map_err(|e| e.to_string())?;

        //
        if let Some(driver_instances) = brut.get("driver_instances") {
            if let Some(driver_instances) = driver_instances.as_object() {
                //
                for (instance_names, body) in driver_instances.iter() {
                    // println!("Key: {}, Value: {}", instance_names, body);

                    self.flatten_value(format!("pza/{}", instance_names), body);
                }
            }
        }

        self.brut = brut;

        // println!("------- {:?}", self.brut);
        // println!("******* {:?}", self.flat);

        self.initialized.notify_waiters();

        Ok(())
    }

    ///
    ///
    pub fn flatten_value(&mut self, level: String, data: &JsonValue) {
        if let Some(data) = data.as_object() {
            // println!("flatten_value: {:?} {:?}", level, data);

            self.flatten_object(level, data).unwrap();
        }
    }

    ///
    ///
    pub fn flatten_object(
        &mut self,
        level: String,
        data: &Map<String, JsonValue>,
    ) -> Result<(), String> {
        //
        //
        match data.get("attributes") {
            Some(att_values) => {
                let values = att_values
                    .as_object()
                    .ok_or("'attributes' is not an object")?;

                // println!("flatten_object: {:?} {:?}", level, values);

                for (att_name, att_data) in values.iter() {
                    self.register_flat_entry(format!("{}/{}", level, att_name), att_data)?;
                }
            }
            None => {}
        }

        //
        //
        match data.get("classes") {
            Some(classes) => {
                let values = classes.as_object().ok_or("'classes' is not an object")?;

                // println!("flatten_object: {:?} {:?}", level, values);

                for (att_name, att_data) in values.iter() {
                    self.flatten_value(format!("{}/{}", level, att_name), att_data);
                }
            }
            None => {}
        }
        // for c_name, c_data in classes.items():
        //     self.flatten_structure(f"{level}/{c_name}", c_data)

        Ok(())
    }

    /// Register a entry inside the flat structure
    ///
    pub fn register_flat_entry(&mut self, level: String, data: &JsonValue) -> Result<(), String> {
        // ultra trace
        // println!("register_flat_entry: {:?}", level);

        // insert the element
        self.flat.insert(
            level.clone(),
            AttributeMetadata::from_json_value(level, &data)?,
        );

        Ok(())
        // .ok_or(format!("cannot insert entry {:?}", &level))
        // .map(|_| ())
    }

    pub fn find_attribute<A: Into<String>>(&self, pattern: A) -> Option<AttributeMetadata> {
        let a: String = pattern.into();

        //
        let p = Pattern::parse(without_escape(a.as_str())).unwrap();
        // assert_eq!(p.find("string"), Some(2..6));

        for (topic, metadata) in self.flat.iter() {
            if p.find(&topic).is_some() {
                return Some(metadata.clone());
            }
        }

        None
    }

    ///
    ///
    pub fn initialized_notifier(&self) -> Arc<Notify> {
        self.initialized.clone()
    }

    /// Debug fonction to list all the received topics
    ///
    pub fn list_of_registered_topics(&self) -> Vec<String> {
        self.flat.keys().cloned().collect()
    }
}

#[derive(Clone, Debug)]
/// Object to manage the structure
///
pub struct Structure {
    /// initial data
    ///
    value: Arc<Mutex<StructureData>>,
}

impl Structure {
    pub fn new(mut data_receiver: DataReceiver) -> Self {
        let json_value = Arc::new(Mutex::new(StructureData::new()));

        let json_value_2 = json_value.clone();
        tokio::spawn(async move {
            loop {
                let message = data_receiver.recv().await;
                // println!("!!!!!!!!!!! ssss ttt Notification = {:?}", message);

                if let Some(message) = message {
                    match json_value_2.lock() {
                        Ok(mut deref_value) => {
                            deref_value.update(message).unwrap();
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

    /// Try to find the element in the structure
    ///
    pub fn find_attribute<A: Into<String>>(&self, name: A) -> Option<AttributeMetadata> {
        match self.value.lock() {
            Ok(v) => v.find_attribute(name),
            Err(_) => None,
        }
    }

    /// Debug fonction to list all the received topics
    ///
    pub fn list_of_registered_topics(&self) -> Vec<String> {
        self.value.lock().unwrap().list_of_registered_topics()
    }

    // pub fn update(&mut self, json_value: JsonValue) {
    //     self.json_value = json_value;
    // }

    // pub async fn run(&mut self) {

    // }

    ///
    ///
    pub fn initialized_notifier(&self) -> Arc<Notify> {
        self.value.lock().unwrap().initialized_notifier()
    }
}
