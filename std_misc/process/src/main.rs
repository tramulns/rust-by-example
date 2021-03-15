use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| panic!("Ошибка выполнения процесса {}", e));

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc завершился успешно и вывел в stdout:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc завершился с ошибкой и вывел в stderr:\n{}", s);
    }
}
