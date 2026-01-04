use std::time::Duration;

use reqwest;
use reqwest::{Client, ClientBuilder, Response};
use reqwest::Url;

use crate::errors::WledJsonApiError;
use crate::structures::cfg::Cfg;
use crate::structures::effects::Effects;
use crate::structures::info::Info;
use crate::structures::live::Live;
use crate::structures::net::Net;
use crate::structures::nodes::Nodes;
use crate::structures::palettes::Palettes;
use crate::structures::state::State;

#[derive(Debug)]
pub struct Wled {
    pub effects: Option<Effects>,
    pub palettes: Option<Palettes>,
    pub state: Option<State>,
    pub info: Option<Info>,
    pub cfg: Option<Cfg>,
    pub live: Option<Live>,
    pub nodes: Option<Nodes>,
    pub net: Option<Net>,
    pub client: Client,
    pub url: Url,
}

impl Wled {
    pub async fn try_from_url(url: &Url) -> Result<Wled, WledJsonApiError> {
        let temp_client: Client = ClientBuilder::new()
            .gzip(true)
            .timeout(Duration::from_millis(5000u64))
            .build()
            .map_err(|e| WledJsonApiError::ReqwestError(e))?;

        let mut temp_url: Url = url.clone();
        temp_url.set_path("json/cfg");

        match temp_client
            .get(temp_url.clone())
            .send()
            .await
        {
            Ok(a) if a.status() == reqwest::StatusCode::OK => {
                Ok(Wled {
                    effects: None,
                    palettes: None,
                    state: None,
                    info: None,
                    cfg: None,
                    live: None,
                    nodes: None,
                    net: None,
                    client: temp_client,
                    url: temp_url,
                })
            }
            Ok(o) => Err(WledJsonApiError::HttpError(o.status())),
            Err(e) => Err(WledJsonApiError::ReqwestError(e))
        }
    }

    pub async fn try_from_client(url: &Url, client: Client) -> Result<Wled, WledJsonApiError> {

        let mut temp_url: Url = url.clone();
        temp_url.set_path("json/cfg");

        match client
            .get(temp_url.clone())
            .send()
            .await
        {
            Ok(a) if a.status() == reqwest::StatusCode::OK => {
                Ok(Wled {
                    effects: None,
                    palettes: None,
                    state: None,
                    info: None,
                    cfg: None,
                    live: None,
                    nodes: None,
                    net: None,
                    client,
                    url: temp_url,
                })
            }
            Ok(o) => Err(WledJsonApiError::HttpError(o.status())),
            Err(e) => Err(WledJsonApiError::ReqwestError(e))
        }
    }

    pub async fn flush_state(&self) -> Result<Response, WledJsonApiError> {
        match &self.state {
            Some(s) => {
                let packet: String = s.try_into()?;

                let mut temp_url = self.url.clone();
                temp_url.set_path("json/state");

                Ok(self.client.post(temp_url).body(packet).send().await?)
            }
            None => Err(WledJsonApiError::FlushNone)
        }
    }

    /// be careful with this, this library does not stop you from sending invalid and crazy configs.
    /// as long as the fields make sense it should work, but
    pub async fn flush_config(&self) -> Result<Response, WledJsonApiError> {
        match &self.cfg {
            Some(s) => {
                let packet: String = s.try_into()?;

                let mut temp_url = self.url.clone();
                temp_url.set_path("json/cfg");

                Ok(self.client.post(temp_url).body(packet).send().await?)
            }
            None => Err(WledJsonApiError::FlushNone)
        }
    }

    pub async fn get_effects_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/eff");
        self.effects = Some(Effects::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_info_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/info");
        self.info = Some(Info::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_state_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/state");
        self.state = Some(State::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_cfg_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/cfg");
        self.cfg = Some(Cfg::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_net_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/net");
        self.net = Some(Net::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_nodes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/nodes");
        self.nodes = Some(Nodes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_palettes_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/pal");
        self.palettes = Some(Palettes::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }

    pub async fn get_live_from_wled(&mut self) -> Result<(), WledJsonApiError> {
        let mut temp_url = self.url.clone();
        temp_url.set_path("json/live");
        self.live = Some(Live::try_from(
            &*self.client
                .get(temp_url)
                .header(reqwest::header::ACCEPT, "application/json")
                .send()
                .await?
                .text()
                .await?
        )?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::structures::cfg::cfg_def::Def;
    use super::*;

    #[tokio::test]
    async fn it_works() {
        // create the URL
        let url: Url = Url::try_from("http://192.168.1.40/").unwrap();

        // create the WLED connection
        let mut wled: Wled = Wled::try_from_url(&url).await.unwrap();
        println!("new wled: {wled:?}");

        // turn off the WLED
        {
            // put the desired change in the internal state data member
            wled.state = Some(State {
                on: Some(true),
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

            // flush and print the server response
            let response = wled.flush_state().await.unwrap();
            println!("turning the thing off {:?}", response.text().await);
        }

        // fill internal cfg with result from WLED
        // Note: These must still be sequential because they mutate `self`.
        wled.get_cfg_from_wled().await.unwrap();
        wled.get_effects_from_wled().await.unwrap();
        wled.get_live_from_wled().await.unwrap();
        wled.get_net_from_wled().await.unwrap();
        wled.get_nodes_from_wled().await.unwrap();
        wled.get_palettes_from_wled().await.unwrap();
        wled.get_state_from_wled().await.unwrap();
        wled.get_info_from_wled().await.unwrap();
    }
}