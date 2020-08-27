

use std::collections::HashMap;
use crate::{util::AircraftConfiguration, NetworkClient, ATCPosition, PilotPosition};

#[derive(Debug)]
pub struct Pilot {
    client: Option<NetworkClient>,
    config: Option<AircraftConfiguration>,
    position: Option<PilotPosition>
}

impl Default for Pilot {
    fn default() -> Self {
        Self {
            client: None,
            config: None,
            position: None,
        }
    }
}

#[derive(Debug)]
pub struct PilotManager {
    pilots: HashMap<String, Pilot>
}

impl PilotManager {
    pub fn new() -> Self { 
        Self { 
            pilots: HashMap::new()
        } 
    }
    

    pub fn process_client(&mut self, client: &NetworkClient) {
        if let Some(data) = self.pilots.get_mut(&client.callsign) {
            data.client = Some(client.clone());
        } else {
            self.pilots.insert(client.callsign.to_string(), Pilot {client: Some(client.clone()), ..Default::default()});
        }
    }
    
    pub fn process_position(&mut self, position: &PilotPosition) {
        if let Some(data) = self.pilots.get_mut(&position.callsign) {
            data.position = Some(position.clone());
        } else {
            self.pilots.insert(position.callsign.to_string(), Pilot {position: Some(position.clone()), ..Default::default()});
        }
    }

    pub fn process_config(&mut self, callsign: &String, aircraft_config: &AircraftConfiguration) {
        if let Some(data) = self.pilots.get_mut(callsign) {
            data.config = Some(aircraft_config.clone());
        } else {
            self.pilots.insert(callsign.to_string(), Pilot {config: Some(aircraft_config.clone()), ..Default::default()});
        }
    }

    pub fn get_client(&self, callsign: &String) -> Option<NetworkClient> {
        if let Some(pilot) = self.pilots.get(callsign) {
            return pilot.client.clone();
        }
        None
    }

    pub fn get_position(&self, callsign: &String) -> Option<PilotPosition> {
        if let Some(pilot) = self.pilots.get(callsign) {
            return pilot.position.clone();
        }
        None
    }

    pub fn get_config(&self, callsign: &String) -> Option<AircraftConfiguration> {
        if let Some(pilot) = self.pilots.get(callsign) {
            return pilot.config.clone();
        }
        None
    }

    pub fn number_tracked(&self) -> usize {
        return self.pilots.len();
    }

    pub fn delete(&mut self, callsign: &String) {
        self.pilots.remove(callsign);
    }
}

#[derive(Debug)]
pub struct ATC {
    client: Option<NetworkClient>,
    position: Option<ATCPosition>
}

#[derive(Debug)]
pub struct ATCManager {
    atc: HashMap<String, ATC>
}

impl ATCManager {
    pub fn new() -> Self {
        Self {
            atc: HashMap::new()
        }
    }

    pub fn process_client(&mut self, client: &NetworkClient) {
        if let Some(data) = self.atc.get_mut(&client.callsign) {
            data.client = Some(client.clone());
        } else {
            self.atc.insert(client.callsign.to_string(), ATC {client: Some(client.clone()), position: None});
        }
    }
    
    pub fn process_position(&mut self, position: &ATCPosition) {
        if let Some(data) = self.atc.get_mut(&position.callsign) {
            data.position = Some(position.clone());
        } else {
            self.atc.insert(position.callsign.to_string(), ATC {client: None, position: Some(position.clone())});
        }
    }

    pub fn get_client(&self, callsign: &String) -> Option<NetworkClient> {
        if let Some(atc) = self.atc.get(callsign) {
            return atc.client.clone();
        }
        None
    }

    pub fn get_position(&self, callsign: &String) -> Option<ATCPosition> {
        if let Some(atc) = self.atc.get(callsign) {
            return atc.position.clone();
        }
        None
    }

    pub fn number_tracked(&self) -> usize {
        return self.atc.len();
    }

    pub fn delete(&mut self, callsign: &String) {
        self.atc.remove(callsign);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fsdpackets::*;

    macro_rules! get_test_pilot {
        () => {
            NetworkClient {
                client_type: NetworkClientType::Pilot,
                callsign: "DAL512".to_string(),
                real_name: "Test".to_string(),
                cid: "3210".to_string(),
                password: "".to_string(),
                rating: NetworkRating::OBS,
                protocol_ver: 100,
            }
        };
    }

    #[test]
    fn test_pilot_client() {
        let mut manager = PilotManager::new();
        let pilot = get_test_pilot!();
        manager.process_client(&pilot);
        assert!(manager.pilots.len() > 0);
        let result = manager.get_client(&pilot.callsign).unwrap();
        assert_eq!(result.rating, NetworkRating::OBS);

        assert_eq!(manager.get_client(&"www".to_string()), None);
    }
    
    #[test]
    fn test_pilot_remove() {
        let mut manager = PilotManager::new();
        let pilot = get_test_pilot!();
        manager.process_client(&pilot);
        manager.delete(&pilot.callsign);
        assert_eq!(manager.get_client(&pilot.callsign), None);
    }
}