use std::process::Command;


pub fn launch(program: &str){
    let _ = Command::new(program)
    .spawn();
}
// pub fn open_vscode(){
//     let _ = Command::new("code")
//         .spawn();
// }

// pub fn open_firefox(){
//     let _ = Command::new("firefox")
//         .spawn();
// }

// pub fn shutdown(){
//     let _ = Command::new("shutdown")
//         .arg("-h")
//         .arg("now")
//         .spawn();
// }

// pub fn restart(){
//     let _ = Command::new("reboot")
//     .arg("-r")
//     .arg("now")
//     .spawn();
// }

// pub fn sleep(){
//     let _ = Command::new("systemctl")
//         .arg("suspend")
//         .spawn();
// }

// pub fn open_terminal(){
//     if Command::new("kitty").spawn().is_err() {
//         let _ = Command::new("konsole").spawn();
//     }
// }