use std::str::FromStr;
use itertools::Itertools;
use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::axis::{ChannelAxis, CustomAxis, SpaceAxis, TimeAxis};
use crate::util::{IsValid, warn_unless};
use crate::multiscale::Axes;

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Multiscale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub axes: Vec<String>,

    pub datasets: Vec<Dataset>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downscaling_type: Option<String>,

    // fields in metadata depend on `downscaling_type`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl Axes for Multiscale {
    fn get_space_axes(&self) -> Vec<SpaceAxis> {
        self.axes
            .iter()
            .filter(|a| vec!["x", "y", "z"].contains(&a.as_str()))
            .map(|a| SpaceAxis::from_str(a).unwrap())
            .collect()
    }

    fn get_time_axes(&self) -> Vec<TimeAxis> {
        if self.axes.contains(&"t".to_string()) {
            vec![TimeAxis::from_str("t").unwrap()]
        } else {
            Vec::new()
        }
    }

    fn get_channel_axes(&self) -> Vec<ChannelAxis> {
        if self.axes.contains(&"c".to_string()) {
            vec![ChannelAxis::from_str("c").unwrap()]
        } else {
            Vec::new()
        }
    }

    // no custom axes allowed
    fn get_custom_axes(&self) -> Vec<CustomAxis> {
        Vec::new()
    }

    fn get_time_axis(&self) -> Option<TimeAxis> {
        if self.axes.contains(&"t".to_string()) {
            Some(TimeAxis::from_str("t").unwrap())
        } else {
            None
        }
    }

    fn get_channel_axis(&self) -> Option<ChannelAxis> {
        if self.axes.contains(&"c".to_string()) {
            Some(ChannelAxis::from_str("c").unwrap())
        } else {
            None
        }
    }

    fn get_custom_axis(&self) -> Option<CustomAxis> {
        None
    }
}

impl IsValid for Multiscale {
    fn is_valid(&self) -> bool {
        self.are_axis_names_valid()
    }
}

impl Multiscale {
    fn are_axis_names_unique(&self) -> bool {
        warn_unless!(
            self.axes.iter().unique().count() == self.axes.len(),
            "The spec states: The values MUST be unique."
        )
    }

    fn are_axis_names_known(&self) -> bool {
        warn_unless!(
            self.axes.iter().all(|a| vec!["t", "c", "z", "y", "x"].contains(&a.as_str())),
            "The spec states: The values MUST be [...] one of {\"t\", \"c\", \"z\", \"y\", \"x\"}."
        )
    }

    fn are_axis_names_valid(&self) -> bool {
         self.are_axis_names_known() && self.are_axis_names_unique()
    }
}
