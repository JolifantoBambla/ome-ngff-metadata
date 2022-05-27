use crate::axis::{ChannelAxis, CustomAxis, SpaceAxis, TimeAxis};
use crate::util::{IsValid, versioned};

pub trait Axes {
   fn get_space_axes(&self) -> Vec<SpaceAxis> {
      Vec::new()
   }

   fn get_time_axes(&self) -> Vec<TimeAxis> {
      Vec::new()
   }

   fn get_channel_axes(&self) -> Vec<ChannelAxis> {
      Vec::new()
   }

   fn get_custom_axes(&self) -> Vec<CustomAxis> {
      Vec::new()
   }

   fn get_time_axis(&self) -> Option<TimeAxis> {
      None
   }

   fn get_channel_axis(&self) -> Option<ChannelAxis> {
      None
   }

   fn get_custom_axis(&self) -> Option<CustomAxis> {
      None
   }
}

pub mod v0_2;
pub mod v0_3;
pub mod v0_4;

versioned!(Multiscale {
   V0_4(v0_4::Multiscale : "0.4"),
   V0_3(v0_3::Multiscale : "0.3"),
   V0_2(v0_2::Multiscale : "0.2"),
});

impl IsValid for Multiscale {
   fn is_valid(&self) -> bool {
      match self {
         Multiscale::V0_4(m) => m.is_valid(),
         Multiscale::V0_3(m) => m.is_valid(),
         Multiscale::V0_2(m) => m.is_valid(),
      }
   }
}

impl Axes for Multiscale {
   fn get_space_axes(&self) -> Vec<SpaceAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_space_axes(),
         Multiscale::V0_3(m) => m.get_space_axes(),
         Multiscale::V0_2(m) => m.get_space_axes(),
      }
   }

   fn get_time_axes(&self) -> Vec<TimeAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_time_axes(),
         Multiscale::V0_3(m) => m.get_time_axes(),
         Multiscale::V0_2(m) => m.get_time_axes(),
      }
   }

   fn get_channel_axes(&self) -> Vec<ChannelAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_channel_axes(),
         Multiscale::V0_3(m) => m.get_channel_axes(),
         Multiscale::V0_2(m) => m.get_channel_axes(),
      }
   }

   fn get_custom_axes(&self) -> Vec<CustomAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_custom_axes(),
         Multiscale::V0_3(m) => m.get_custom_axes(),
         Multiscale::V0_2(m) => m.get_custom_axes(),
      }
   }

   fn get_time_axis(&self) -> Option<TimeAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_time_axis(),
         Multiscale::V0_3(m) => m.get_time_axis(),
         Multiscale::V0_2(m) => m.get_time_axis(),
      }
   }

   fn get_channel_axis(&self) -> Option<ChannelAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_channel_axis(),
         Multiscale::V0_3(m) => m.get_channel_axis(),
         Multiscale::V0_2(m) => m.get_channel_axis(),
      }
   }

   fn get_custom_axis(&self) -> Option<CustomAxis> {
      match self {
         Multiscale::V0_4(m) => m.get_custom_axis(),
         Multiscale::V0_3(m) => m.get_custom_axis(),
         Multiscale::V0_2(m) => m.get_custom_axis(),
      }
   }
}
