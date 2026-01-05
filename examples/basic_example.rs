use std::time::Duration;
use reqwest::Url;
use wled_json_api_library::wled::Wled;
use wled_json_api_library::structures::{state::State, cfg::Cfg, cfg::cfg_def::Def};

#[tokio::main]
async fn main() {

    // create the URL
    let url: Url = Url::try_from("http://192.168.1.209/").unwrap();

    // create the WLED connection
    let mut wled: Wled = Wled::try_from_url(&url).await.unwrap();
    println!("new wled: {wled:?}");

    // turn off the WLED
    {
        // put the desired change in the internal state data member
        wled.state = Some(State {
            on: Some(false),
            bri: None,
            transition: None,
            tt: None,
            ps: None,
            psave: None,
            pl: None,
            nl: None,
            udpn: None,
            v: None,
            rb: None,
            live: None,
            lor: None,
            time: None,
            mainseg: None,
            playlist: None,
            seg: None,
        });

        // or use this nice syntax
        wled.state = Some(State {
            on: Some(false),
            ..Default::default()
        });

        // flush and print the server response
        let response = wled.flush_state().await.unwrap();
        println!("turning the thing off {:?}", response.text().await.unwrap());
    }


    // fill internal cfg with result from WLED
    wled.get_cfg_from_wled().await.unwrap();

    // get the field defining the power on boot default behavior
    let turn_on_after_boot = wled.cfg.unwrap().def.unwrap().on.unwrap();
    // print it
    println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);


    // put the desired change into the config data member
    wled.cfg = Some(Cfg{
        def: Some(Def{
            ps: None,
            on: Some(!turn_on_after_boot),
            bri: None,
        }),
        ..Default::default()
    });

    // print the response.
    let response = wled.flush_config().await.unwrap();
    println!("toggling: {:?}", response.text().await.unwrap());

    // wait for WLED to finish making this change.
    // Around 100 milliseconds should be enough on good hardware,
    // but this is especially slow because it has to read and write from the internal filesystem
    // where the config file is stored
    tokio::time::sleep(Duration::from_millis(80)).await;


    // get and print the new state from the server
    wled.get_cfg_from_wled().await.unwrap();
    let new_cfg = wled.cfg.clone().unwrap();
    let turn_on_after_boot = new_cfg.def.unwrap().on.unwrap();

    println!("received cfg, turn on after boot: {:?}", turn_on_after_boot);

    // turn on the WLED
    {

        // or use this nice syntax
        wled.state = Some(State {
            on: Some(true),
            ..Default::default()
        });

        // flush and print the server response
        let response = wled.flush_state().await.unwrap();
        println!("turning the thing back on {:?}", response.text().await.unwrap());
    }


}
