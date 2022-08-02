use anyhow::Result;
use futures::TryStreamExt;
use k8s_openapi::api::core::v1::PersistentVolume;

use kube::{
    api::{Api, ListParams},
    runtime::{watcher, WatchStreamExt},
    Client,
};
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::IoEngineApiClient;

const MAYASTOR_PLUGIN: &str = "io.openebs.csi-mayastor";
const BOLT_PLUGIN: &str = "com.datacore.bolt";

pub struct PvGarbageCollector {}

impl PvGarbageCollector {
    pub async fn start_watcher() -> Result<(), watcher::Error> {
        info!("Starting PV Garbage Collector");
        debug!("Handling if any missed events");
        tokio::spawn(async {
            handle_missed_events().await;
        });
        let client = Client::try_default().await.unwrap();
        let pv_handle = Api::<PersistentVolume>::all(client.clone());
        watcher(pv_handle, ListParams::default())
            .touched_objects()
            .try_for_each(|pvol| async {
                process_object(pvol).await;
                Ok(())
            })
            .await
            .unwrap();
        Ok(())
    }
}

///Accepts PV object received from event stream and applies Garbage collector logic.
async fn process_object(pv: PersistentVolume) {
    let client = Client::try_default().await.unwrap();
    let pv_handle = Api::<PersistentVolume>::all(client.clone());
    let mut deletion_candidate = false;
    let mut our_pv = false;
    match pv.metadata.clone().deletion_timestamp {
        Some(_) => deletion_candidate = true,
        None => {}
    }

    if let Some(provisioner) = &pv.spec.as_ref().unwrap().csi {
        if provisioner.driver == MAYASTOR_PLUGIN || provisioner.driver == BOLT_PLUGIN {
            our_pv = true;
        }
    }

    if deletion_candidate && our_pv {
        if let Some(reclaim_policy) = &pv.spec.as_ref().unwrap().persistent_volume_reclaim_policy {
            if let Some(phase) = &pv.status.as_ref().unwrap().phase {
                if reclaim_policy == "Retain" && phase == "Released" {
                    debug!(
                        "PV :{} is a deletion candidate",
                        &pv.metadata.clone().name.unwrap()
                    );
                    let vol_handle: &String = &pv
                        .spec
                        .as_ref()
                        .unwrap()
                        .csi
                        .as_ref()
                        .unwrap()
                        .volume_handle;
                    delete_volume(vol_handle).await;
                }
                if phase == "Bound" {
                    if let Ok(pvol) = pv_handle.get_opt(&pv.metadata.clone().name.unwrap()).await {
                        match pvol {
                            Some(_) => debug!(
                                "PV {} present on API server",
                                &pv.metadata.clone().name.unwrap()
                            ),
                            None => {
                                debug!(
                                    "PV :{} is a deletion candidate",
                                    &pv.metadata.clone().name.unwrap()
                                );
                                let vol_handle: &String = &pv
                                    .spec
                                    .as_ref()
                                    .unwrap()
                                    .csi
                                    .as_ref()
                                    .unwrap()
                                    .volume_handle;
                                delete_volume(vol_handle).await;
                            }
                        }
                    } else {
                        debug!("Unknown error");
                    }
                }
            }
        }
    }
}

///Accepts volume id and calls Control plane api to delete the Volume
async fn delete_volume(vol_handle: &str) {
    let volume_uuid = Uuid::parse_str(vol_handle).unwrap();
    if IoEngineApiClient::get_client()
        .delete_volume(&volume_uuid)
        .await
        .is_err()
    {
        error!("Failed to delete volume {}", volume_uuid);
    } else {
        info!("Successfully deleted volume {}", volume_uuid);
    }
}

///Handle if there is any missed events at startup.
async fn handle_missed_events() {
    let client = Client::try_default().await.unwrap();
    let pv_handle = Api::<PersistentVolume>::all(client.clone());
    if let Ok(volume_list) = IoEngineApiClient::get_client()
        .list_volumes(0, "".to_string())
        .await
    {
        for vol in volume_list.entries {
            let pv = "pvc-".to_string() + &vol.spec.uuid.to_string();
            if let Ok(pvol) = pv_handle.get_opt(&pv).await {
                match pvol {
                    Some(_) => {}
                    None => {
                        debug!("PV :{} is a deletion candidate", pv);
                        let vol_handle = &vol.spec.uuid.to_string();
                        delete_volume(vol_handle).await;
                    }
                }
            }
        }
    } else {
        error!("Couldnt get volume list");
    }
}
