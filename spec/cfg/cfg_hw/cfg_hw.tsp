import "@typespec/json-schema";

using TypeSpec.JsonSchema;

import "./cfg_hw_led.tsp";


/// Hardware info
@jsonSchema
model Hw {
  /// Led info
  led?: Led;

  /// List of color order maps
  com?: ColorOrderMap[];

  /// Info about connected buttons
  btn?: Btn;

  /// info about IR receiver
  ir?: Ir;

  /// info about relay
  relay?: Relay;

  /// baud rate/100
  baud?: uint16;

  /// interface info
  @encodedName("application/json", "if")
  `if`?: If;
}


@jsonSchema
model Btn {
  /// just information about max number of buttons (not actually used)
  max?: uint8;

  /// idfk
  pull?: boolean;

  /// balls, testicles
  ins?: In3[];

  /// Touch threshold
  tt?: uint8;

  /// button Publish Mqtt
  mqtt?: boolean;
}

@jsonSchema
model Relay {
  /// pin for the relay
  pin?: int8;

  /// negate output
  rev?: boolean;
}




@jsonSchema
model ColorOrderMap {
  /// Probably the start index?
  start?: uint16;

  /// stop index? probably?, could not find in latest source, but its in the WLED 14.0 binary
  @jsonSchema(writeonly: true)
  stop?: uint16;

  /// length ig?
  len?: uint16;

  /// Whatever the fuck 'order' is
  order?: uint8;
}


@jsonSchema
model ButtonMacros {
  /// preset to apply when button is pressed once, or default behaviour if 0.
  @encodedName("application/json", "0")
  macro_button?: uint8;

  /// preset to apply when button is pressed for a long time, or default behaviour if 0.
  @encodedName("application/json", "1")
  macro_long_press?: uint8;

  /// preset to apply when button is pressed twice, or default behaviour if 0.
  @encodedName("application/json", "2")
  macro_double_press?: uint8;
}


@jsonSchema
model Ir {
  /// Pin for this sensor. presumably fucking explodes if negative
  pin?: int8;

  /// IR type 0 is disabled, go scour the undocumented WLED source code for the rest
  @encodedName("application/json", "type")
  `type`?: uint8;

  /// apply IR to all selected segments; who knows what happens when false
  sel?: boolean;
}


@jsonSchema
model If {
  /// [i2c_sda pin, i2c_scl pin]
  @encodedName("application/json", "i2c-pin")
  i2c_pin?: [int8, int8];

  /// [spi_mosi pin, spi_sclk pin, spi_miso pin]
  @encodedName("application/json", "spi-pin")
  spi_pin?: [int8, int8, int8];
}


@jsonSchema
model In3 {
  /// Button type
  @encodedName("application/json", "type")
  `type`?: ButtonType;

  /// Button pin
  pin?: int8[];

  /// Button macros
  macros?: ButtonMacros;
}



/// Various types
enum ButtonType {
  /// None
  BTN_TYPE_NONE: 0,
  /// Reserved
  BTN_TYPE_RESERVED: 1,
  /// Push button that is considered "pushed" when at logic low
  BTN_TYPE_PUSH: 2,
  /// Push button that is considered "pushed" when at logic high
  BTN_TYPE_PUSH_ACT_HIGH: 3,
  /// A switch with no defined on or off
  BTN_TYPE_SWITCH: 4,
  /// PIR sensor
  BTN_TYPE_PIR_SENSOR: 5,
  /// Touch sensor (presumably using the built in touch sensor of the ESP32)
  BTN_TYPE_TOUCH: 6,
  /// Not really a button, but there you go
  BTN_TYPE_ANALOG: 7,
  /// BTN_TYPE_ANALOG, but inverted
  BTN_TYPE_ANALOG_INVERTED: 8,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD1: 9,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD2: 10,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD3: 11,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD4: 12,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD5: 13,
  /// Reserved to keep some semblance of backwards compatibility when new WLED versions come out with more AP behaviours
  RSVD6: 14,
}
