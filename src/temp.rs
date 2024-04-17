use notify_rust::Notification;
use mpris::PlayerFinder;
use std::{thread::sleep, time::Duration};
fn main(){
    let player = PlayerFinder::new()
        .expect("no dbus :0")
        .find_active()
        .expect("no player eh");
    loop {
        let metadata_before = player.get_metadata().unwrap();
        sleep(Duration::from_millis(50));
        let metadata_after = player.get_metadata().unwrap();
        if metadata_before.track_id().unwrap() != metadata_after.track_id().unwrap() && metadata_after.title().unwrap().len() != 0 {
            let _ = Notification::new()
            .summary("Now playing",)
            .body(metadata_after.title().unwrap())
            .icon(metadata_after.art_url().unwrap())
            .show();
        }
        else {continue;}
    }
    
    
}
