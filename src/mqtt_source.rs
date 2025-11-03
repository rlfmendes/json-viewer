use crate::data_source::{DataEntry, DataSource};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use rumqttc::{Client, Event, MqttOptions, Packet, QoS};
use std::any::Any;
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MqttMessage {
    pub id: String,
    pub topic: String,
    pub payload: String,
    pub timestamp: DateTime<Local>,
    pub qos: QoS,
}

pub struct MqttSource {
    messages: Arc<Mutex<VecDeque<MqttMessage>>>,
    data_entries: Vec<DataEntry>,
    selected_index: usize,
    scroll_offset: usize,
    max_messages: usize,
    broker_url: String,
    topic: String,
    _client: Client,
    update_rx: Receiver<()>,
    pub connection_status: String,
}

impl MqttSource {
    pub fn new(
        broker_url: &str,
        topic: &str,
        username: Option<String>,
        password: Option<String>,
        client_id: Option<String>,
        max_messages: usize,
    ) -> Result<Self> {
        // Parse broker URL
        let url = broker_url.trim_start_matches("mqtt://");
        let parts: Vec<&str> = url.split(':').collect();
        let host = parts.get(0).ok_or_else(|| anyhow!("Invalid broker URL"))?;
        let port: u16 = if parts.len() > 1 {
            parts[1].parse().unwrap_or(1883)
        } else {
            1883
        };

        // Generate client ID if not provided
        let client_id = client_id.unwrap_or_else(|| format!("json-viewer-{}", Uuid::new_v4()));

        // Setup MQTT options
        let mut mqttoptions = MqttOptions::new(&client_id, host.to_string(), port);
        mqttoptions.set_keep_alive(Duration::from_secs(30));

        if let (Some(user), Some(pass)) = (username, password) {
            mqttoptions.set_credentials(user, pass);
        }

        // Create MQTT client and connection
        let (client, mut connection) = Client::new(mqttoptions, 10);

        // Subscribe to topic
        client.subscribe(topic, QoS::AtMostOnce)?;

        // Create shared message storage
        let messages = Arc::new(Mutex::new(VecDeque::new()));
        let messages_clone = Arc::clone(&messages);

        // Create channel to notify UI of updates
        let (update_tx, update_rx) = channel();

        // Spawn thread to handle MQTT events
        let topic_clone = topic.to_string();
        thread::spawn(move || {
            Self::mqtt_event_loop(connection, messages_clone, update_tx, topic_clone, max_messages);
        });

        Ok(Self {
            messages,
            data_entries: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            max_messages,
            broker_url: broker_url.to_string(),
            topic: topic.to_string(),
            _client: client,
            update_rx,
            connection_status: "Connecting...".to_string(),
        })
    }

    fn mqtt_event_loop(
        mut connection: rumqttc::Connection,
        messages: Arc<Mutex<VecDeque<MqttMessage>>>,
        update_tx: Sender<()>,
        topic: String,
        max_messages: usize,
    ) {
        loop {
            match connection.iter().next() {
                Some(Ok(Event::Incoming(Packet::Publish(publish)))) => {
                    // Only process messages from subscribed topics
                    if Self::topic_matches(&topic, &publish.topic) {
                        if let Ok(payload) = String::from_utf8(publish.payload.to_vec()) {
                            let message = MqttMessage {
                                id: Uuid::new_v4().to_string(),
                                topic: publish.topic.clone(),
                                payload,
                                timestamp: Local::now(),
                                qos: publish.qos,
                            };

                            if let Ok(mut msgs) = messages.lock() {
                                // Add new message at the front
                                msgs.push_front(message);

                                // Remove oldest if exceeding limit
                                while msgs.len() > max_messages {
                                    msgs.pop_back();
                                }
                            }

                            // Notify UI of update
                            let _ = update_tx.send(());
                        }
                    }
                }
                Some(Ok(_)) => {}
                Some(Err(e)) => {
                    eprintln!("MQTT connection error: {}", e);
                    thread::sleep(Duration::from_secs(5));
                }
                None => break,
            }
        }
    }

    fn topic_matches(subscription: &str, topic: &str) -> bool {
        // Simple topic matching (supports + and # wildcards)
        let sub_parts: Vec<&str> = subscription.split('/').collect();
        let topic_parts: Vec<&str> = topic.split('/').collect();

        let mut i = 0;
        for (j, sub_part) in sub_parts.iter().enumerate() {
            if *sub_part == "#" {
                return true; // Multi-level wildcard matches everything
            }

            if i >= topic_parts.len() {
                return false;
            }

            if *sub_part != "+" && *sub_part != topic_parts[i] {
                return false;
            }

            i += 1;

            // If we're at the last subscription part and not all topic parts consumed
            if j == sub_parts.len() - 1 && i < topic_parts.len() {
                return false;
            }
        }

        i == topic_parts.len()
    }

    fn update_entries(&mut self) {
        if let Ok(msgs) = self.messages.lock() {
            self.data_entries = msgs
                .iter()
                .map(|msg| {
                    let time_str = msg.timestamp.format("%H:%M:%S").to_string();
                    let preview = if msg.payload.len() > 40 {
                        format!("{}...", &msg.payload[..40])
                    } else {
                        msg.payload.clone()
                    };

                    DataEntry {
                        id: msg.id.clone(),
                        display_name: format!("[{}] {} - {}", time_str, msg.topic, preview),
                        is_navigable: false,
                        metadata: None,
                    }
                })
                .collect();
        }
    }

    pub fn check_updates(&mut self) {
        // Non-blocking check for updates
        while self.update_rx.try_recv().is_ok() {
            self.update_entries();
            self.connection_status = format!(
                "Connected | Topic: {} | Messages: {}",
                self.topic,
                self.data_entries.len()
            );
        }
    }

    pub fn get_message_count(&self) -> usize {
        if let Ok(msgs) = self.messages.lock() {
            msgs.len()
        } else {
            0
        }
    }
}

impl DataSource for MqttSource {
    fn get_entries(&self) -> &[DataEntry] {
        &self.data_entries
    }

    fn get_content(&self, id: &str) -> Result<String> {
        if let Ok(msgs) = self.messages.lock() {
            if let Some(msg) = msgs.iter().find(|m| m.id == id) {
                return Ok(msg.payload.clone());
            }
        }
        Err(anyhow!("Message not found"))
    }

    fn entry_count(&self) -> usize {
        self.data_entries.len()
    }

    fn next(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            self.selected_index = (self.selected_index + 1) % count;
        }
    }

    fn previous(&mut self) {
        let count = self.entry_count();
        if count > 0 {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            } else {
                self.selected_index = count - 1;
            }
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    fn ensure_visible(&mut self, visible_height: usize) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index.saturating_sub(visible_height - 1);
        }
    }

    fn get_selected_entry(&self) -> Option<&DataEntry> {
        self.data_entries.get(self.selected_index)
    }

    fn supports_navigation(&self) -> bool {
        false // MQTT doesn't support directory navigation
    }

    fn navigate_into(&mut self) -> Result<()> {
        Err(anyhow!("MQTT source does not support navigation"))
    }

    fn navigate_parent(&mut self) -> Result<()> {
        Err(anyhow!("MQTT source does not support navigation"))
    }

    fn has_parent(&self) -> bool {
        false
    }

    fn refresh(&mut self) -> Result<()> {
        self.update_entries();
        Ok(())
    }

    fn get_location(&self) -> String {
        format!("{} ({})", self.broker_url, self.topic)
    }

    fn get_file_path(&self) -> Option<PathBuf> {
        None // MQTT doesn't have file paths
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
