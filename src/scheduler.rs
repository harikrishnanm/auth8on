use clokwerk::{Scheduler, TimeUnits};
use log::info;
use std::{thread, time::Duration};

use crate::utils;
use crate::AppState;

pub fn start(app_state: &'static AppState) {
    info!("Starting scheduler...");
    let mut scheduler = Scheduler::new();
    scheduler.every(1.seconds()).run(|| {
        info!("Scheduled task");
    });
    //let thread_handle = scheduler.watch_thread(Duration::from_millis(100));

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(10));
    }
}
