use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency {
    pub text: String,
}

impl Frequency {
    pub fn from_packet_string(data: &str) -> Self {
        return Frequency {
            text: format!("1{}.{}", &data[0..2], &data[2..]),
        };
    }
}
// All structs related to aircraft configuration
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AircraftLights {
    strobe_on: bool,
    landing_on: bool,
    beacon_on: bool,
    nav_on: bool,
    logo_on: bool,
}

impl Default for AircraftLights {
    fn default() -> Self {
        Self {
            strobe_on: false,
            landing_on: false,
            beacon_on: false,
            nav_on: false,
            logo_on: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AircraftEngine {
    on: bool,
}

impl Default for AircraftEngine {
    fn default() -> Self {
        Self { on: false }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AircraftConfiguration {
    lights: AircraftLights,
    engines: HashMap<String, AircraftEngine>,
    flaps_pct: u64,
    gear_down: bool,
    spoilers_out: bool,
    on_ground: bool,
}

impl Default for AircraftConfiguration {
    fn default() -> Self {
        return Self {
            lights: AircraftLights::default(),
            engines: HashMap::new(),
            flaps_pct: 0,
            gear_down: false,
            spoilers_out: false,
            on_ground: false,
        };
    }
}

impl AircraftConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_from_json(&mut self, v: &Value) {
        if let Some(lights) = v.get("lights") {
            if let Some(strobe) = lights.get("strobe_on") {
                self.lights.strobe_on = strobe.as_bool().unwrap();
            } else if let Some(beacon) = lights.get("beacon_on") {
                self.lights.beacon_on = beacon.as_bool().unwrap();
            } else if let Some(nav) = lights.get("nav_on") {
                self.lights.nav_on = nav.as_bool().unwrap();
            } else if let Some(landing) = lights.get("landing_on") {
                self.lights.landing_on = landing.as_bool().unwrap();
            } else if let Some(logo) = lights.get("logo_on") {
                self.lights.logo_on = logo.as_bool().unwrap();
            }
        } else if let Some(engines) = v.get("engines") {
            for (k, v) in engines.as_object().unwrap() {
                if let Some(engine) = self.engines.get_mut(k) {
                    if let Some(on) = v.get("on") {
                        (*engine).on = on.as_bool().unwrap();
                    }
                } else {
                    self.engines
                        .insert(k.to_string(), serde_json::from_value(v.clone()).unwrap());
                }
            }
        } else if let Some(flaps_pct) = v.get("flaps_pct") {
            self.flaps_pct = flaps_pct.as_u64().unwrap();
        } else if let Some(gear_down) = v.get("gear_down") {
            self.gear_down = gear_down.as_bool().unwrap();
        } else if let Some(spoilers_out) = v.get("spoilers_out") {
            self.spoilers_out = spoilers_out.as_bool().unwrap();
        } else if let Some(on_ground) = v.get("on_ground") {
            self.on_ground = on_ground.as_bool().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_packet_frequency() {
        let freq = Frequency::from_packet_string(&"23950".to_string());
        assert_eq!(freq.text, "123.950");
    }

    #[test]
    fn parse_aircraft_configuration() {
        let mut config: AircraftConfiguration = AircraftConfiguration::new();

        let data_string: Value = serde_json::from_str(
            "{\"config\":{\"lights\":{\"beacon_on\":true}, \"gear_down\": true}}",
        )
        .unwrap();
        config.update_from_json(&data_string["config"]);
        assert_eq!(
            config,
            AircraftConfiguration {
                lights: AircraftLights {
                    beacon_on: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        );
        config.update_from_json(&serde_json::from_str("{\"gear_down\": true}").unwrap());
        assert_eq!(
            config,
            AircraftConfiguration {
                gear_down: true,
                lights: AircraftLights {
                    beacon_on: true,
                    ..Default::default()
                },
                ..Default::default()
            }
        );
    }
}
