use serde;
use serde::{Serialize, Deserialize};
use crate::errors::WledJsonApiError;
use crate::structures::none_function;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    /// On/Off state of the light
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub on: Option<bool>,

    /// Brightness of the light. If on is false, contains last brightness when light was on (aka brightness when on is set to true. Setting bri to 0 is supported but it is recommended to use the range 1-255 and use on: false to turn off. The state response will never have the value 0 for bri.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: Option<u8>,

    /// Duration of the crossfade between different colors/brightness levels. One unit is 100ms, so a value of 4 results in a transition of 400ms.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub transition: Option<u8>,

    /// Similar to transition, but applies to just the current API call. Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tt: Option<u8>,

    /// -1 to 65535; ID of currently set preset. 1~17~ can be used to iterate through presets 1-17.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ps: Option<i32>,

    /// 1 to 16 (250 in 0.11); Save current light config to specified preset slot. Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub psave: Option<u8>,

    /// -1 to 0; 	ID of currently set playlist. For now, this sets the preset cycle feature, -1 is off and 0 is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pl: Option<i8>,

    /// Night light
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub nl: Option<Nl>,

    /// UDP sync
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub udpn: Option<Udpn>,

    /// If set to true in a JSON POST command, the response will contain the full JSON state object. Not included in state response
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub v: Option<bool>,

    /// If set to true, device will reboot immediately. Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rb: Option<bool>,

    /// If set to true, enters realtime mode and blanks the LEDs. The realtime timeout option does not have an effect when this command is used, WLED will stay in realtime mode until the state (color/effect/segments, excluding brightness) is changed. It is expected that {"live":false} is sent once live data sending is terminated. Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub live: Option<bool>,

    /// 0, 1, or 2; Live data override. 0 is off, 1 is override until live data ends, 2 is override until ESP reboot (available since 0.10.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub lor: Option<u8>,

    /// Set module time to unix timestamp. Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub time: Option<u32>,

    /// 0 to info.leds.maxseg-1; Main Segment
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mainseg: Option<u8>,

    /// Custom preset playlists. Not included in state response (available since 0.11.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub playlist: Option<Playlist>,

    /// Array of segment objects; Segments are individual parts of the LED strip. In 0.9.0 this will enables running different effects on differentparts of the strip.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub seg: Option<Vec<Seg>>,
}


impl TryInto<String> for &State{
    type Error = WledJsonApiError;
    fn try_into(self) -> Result<String, WledJsonApiError> {
        serde_json::to_string(self).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}

impl TryFrom<&str> for State{
    type Error = WledJsonApiError;
    fn try_from(str_in: &str) -> Result<State, WledJsonApiError> {
        serde_json::from_str(str_in).map_err(|e| {WledJsonApiError::SerdeError(e)})
    }
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nl {
    /// Nightlight currently active
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub on: Option<bool>,

    /// Duration of nightlight in minutes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dur: Option<u8>,

    /// If true, the light will gradually dim over the course of the nightlight duration. If false, it will instantly turn to the target brightness once the duration has elapsed. Removed in 0.13.0 (use mode instead)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fade: Option<bool>,

    /// 0 to 3; Nightlight mode (0: instant, 1: fade, 2: color fade, 3: sunrise) (available since 0.10.2)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mode: Option<u8>,

    /// Target brightness of nightlight feature
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub tbri: Option<u8>,

    /// -1 to 15300; Remaining nightlight duration in seconds, -1 if not active. Only in state response, can not be set.
    #[serde(skip_serializing)]
    #[serde(default = "none_function")]
    pub rem: Option<i16>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Udpn {

    /// Send WLED broadcast (UDP sync) packet on state change
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub send: Option<bool>,

    /// Receive broadcast packets
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub recv: Option<bool>,

    /// Bitfield for broadcast send groups 1-8
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sgrp: Option<u8>,

    /// Bitfield for broadcast receive groups 1-8
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rgrp: Option<u8>,

    /// Don't send a broadcast packet (applies to just the current API call). Not included in state response.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub nn: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seg {
    /// -1 to info.maxseg -1; Zero-indexed ID of the segment. May be omitted, in that case the ID will be inferred from the order of the segment objects in the seg array. -1 means apply to all selected segments
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub id: Option<i16>,

    /// 0 to info.leds.count -1; LED the segment starts at.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub start: Option<u16>,

    /// 0 to info.leds.count; LED the segment stops at, not included in range. If stop is set to a lower or equal value than start (setting to 0 is recommended), the segment is invalidated and deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub stop: Option<u16>,

    /// 0 to info.leds.count; Length of the segment (stop - start). stop has preference, so if it is included, len is ignored.
    #[serde(skip_serializing)] // this field is ignored if stop is sent, so don't risk sending bad shit
    #[serde(default = "none_function")]
    pub len: Option<u16>,

    /// Grouping (how many consecutive LEDs of the same segment will be grouped to the same color)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub grp: Option<u8>,

    /// Spacing (how many LEDs are turned off and skipped between each group)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub spc: Option<u8>,

    /// -len+1 to len; 	Offset (how many LEDs to rotate the virtual start of the segments, available since 0.13.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub of: Option<i16>,

    /// Turns on and off the individual segment. (available since 0.10.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub on: Option<bool>,

    /// freezes/unfreezes the current effect
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub frz: Option<bool>,

    /// Sets the individual segment brightness (available since 0.10.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub bri: Option<u8>,

    /// 0 to 255 or 1900 to 10091; White spectrum color temperature (available since 0.13.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub cct: Option<u16>,

    /// Undocumented?????? TODO
    ///
    /// // 14-15 : 0-3 UI segment sets/groups
    ///     -WLED source
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub set: Option<u8>,

    /// The name of the segment. Names are not present by default.
    /// if this is none, use "Segment{id}" to match the WLED UI
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    #[serde(rename = "n")]
    pub name: Option<String>,

    /// Array that has up to 3 color arrays as elements, the primary, secondary (background) and tertiary colors of the segment. Each color is an array of 3 or 4 bytes, which represent an RGB(W) color.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub col: Option<Vec<Vec<u8>>>,

    /// 0 to info.fxcount -1; ID of the effect or ~ to increment, ~- to decrement, or r for random.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub fx: Option<u16>,

    /// Relative effect speed. ~ to increment, ~- to decrement. ~10 to increment by 10, ~-10 to decrement by 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sx: Option<u8>,

    /// Effect intensity. ~ to increment, ~- to decrement. ~10 to increment by 10, ~-10 to decrement by 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ix: Option<u8>,

    /// 0 to info.palcount -1; ID of the color palette or ~ to increment, ~- to decrement, or r for random.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub pal: Option<u16>,

    /// Effect custom slider 1. Custom sliders are hidden or displayed and labeled based on effect metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub c1: Option<u8>,

    /// Effect custom slider 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub c2: Option<u8>,

    /// 0 to 31; Effect custom slider 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub c3: Option<u8>,

    /// true if the segment is selected. Selected segments will have their state (color/FX) updated by APIs that don't support segments (e.g. UDP sync, HTTP API). If no segment is selected, the first segment (id:0) will behave as if selected. WLED will report the state of the first (lowest id) segment that is selected to APIs (HTTP, MQTT, Blynk...), or mainseg in case no segment is selected and for the UDP API. Live data is always applied to all LEDs regardless of segment configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub sel: Option<bool>,

    /// Flips the segment, causing animations to change direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub rev: Option<bool>,

    /// Mirrors the segment (available since 0.10.2)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub mi: Option<bool>,

    /// Effect option 1. Custom options are hidden or displayed and labeled based on effect metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub o1: Option<bool>,

    /// Effect option 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub o2: Option<bool>,

    /// Effect option 3.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub o3: Option<bool>,

    /// 0 to 3; Setting of the sound simulation type for audio enhanced effects. (0: 'BeatSin', 1: 'WeWillRockYou', 2: '10_3', 3: '14_3') (as of 0.14.0-b1, there are these 4 types defined)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub si: Option<u8>,

    /// 0 to 4 [map1D2D.count]; Setting of segment field 'Expand 1D FX'. (0: Pixels, 1: Bar, 2: Arc, 3: Corner)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub m12: Option<u8>,
}




#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    /// Array of preset ID integers to be applied in this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub ps: Option<Vec<u8>>,

    /// Array of time each preset should be kept, in tenths of seconds. If only one integer is supplied, all presets will be kept for that time.Defaults to 10 seconds if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub dur: Option<Vec<u32>>,

    /// Array of time each preset should transition to the next one, in tenths of seconds. If only one integer is supplied, all presets will transition for that time. Defaults to the current transition time if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub transition: Option<u8>,

    /// How many times the entire playlist should cycle before finishing. Set to 0 for an indefinite cycle. Default to indefinite if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub repeat: Option<u16>,

    /// Single preset ID to apply after the playlist finished. Has no effect when an indefinite cycle is set. If not provided, the light will stay on the last preset of the playlist.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "none_function")]
    pub end: Option<u8>,
}


#[cfg(test)]
mod tests {
    use crate::structures::state::State;

    #[test]
    fn it_works() {
        let s = r#"{"on":true,"bri":128,"transition":7,"ps":-1,"pl":-1,"nl":{"on":false,"dur":60,"mode":1,"tbri":0,"rem":-1},"udpn":{"send":false,"recv":true,"sgrp":1,"rgrp":1},"lor":0,"mainseg":0,"seg":[{"id":0,"start":0,"stop":6,"len":6,"grp":1,"spc":0,"of":0,"on":true,"frz":false,"bri":255,"cct":127,"set":0,"col":[[255,160,0],[0,0,0],[0,0,0]],"fx":0,"sx":128,"ix":128,"pal":0,"c1":128,"c2":128,"c3":16,"sel":true,"rev":false,"mi":false,"o1":false,"o2":false,"o3":false,"si":0,"m12":0}]}"#;
        println!("og string: {:?}", s);
        let a: &State = &State::try_from(s).unwrap();
        println!("State object: {:?}", a);
        let b: String = a.try_into().unwrap();
        println!("converted object: {:?}", b);


    }
}
