fn main() {
    let f_cpu = std::env::var("F_CPU")
        .unwrap_or_else(|_| "16000000".to_string());

    if f_cpu.parse::<u32>().is_err() {
        panic!("Ошибка: AVR_CPU_FREQUENCY_HZ должна быть числом, а не '{}'", f_cpu);
    }

    // Передаем значение компилятору.
    println!("cargo:rustc-env=F_CPU={}", f_cpu);

    // Указываем Cargo пересобирать проект, если изменился build.rs или переменная
    println!("cargo:rerun-if-env-changed=AVR_CPU_FREQUENCY_HZ");
    println!("cargo:rerun-if-changed=build.rs");
}

