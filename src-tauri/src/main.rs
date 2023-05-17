#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]




fn main() {
    #[cfg(desktop)]
    wind_daily_lib::run();
}
