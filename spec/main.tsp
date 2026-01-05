import "@typespec/json-schema";

using TypeSpec.JsonSchema;

// Import structure modules following the same hierarchy as Rust
import "./cfg/cfg_hw/cfg_hw.tsp";

// Main entrypoint for the WLED JSON API TypeSpec definition
// Additional modules will be imported here as they are converted:
// import "./cfg/mod.tsp";
// import "./effects.tsp";
// import "./info.tsp";
// import "./live.tsp";
// import "./net.tsp";
// import "./nodes.tsp";
// import "./palettes.tsp";
// import "./state.tsp";
// import "./state_info.tsp";
