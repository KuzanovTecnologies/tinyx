use std::{env, error: Error};
fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Pegar o nome do nosso projeto 
    let kernel_name = env::var("CARGO_CFG_TARGET_ARCH")?;
    // Pegar para qual arquitetura estamos compilando o kernel
    let arch = env::var("Cargo_CFG_TARGET_ARCH")?;
    // Fazer o cargo adicionar o linker script certo dependendo da arquitetura 
    match arch.as_str() {
        "x86_64" => {
                println!("cargo:rustc-link-arg-bin={kernel_name}=--script=conf/linker-x86_64.ld");
        
        }
        "aarch64" => {
                println!("cargo:rustc-link-arg-bin={kernel_name}=--script=conf/linker-x86_64.ld");

        }
        other_arch => todo!("{other_arch} is not implemented yet"),
    }
}
// mandar o cargo rodar o nosso build.rs sempre que mudarmos o nome do projeto ou a arquitetura
println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_ARCH");
OK(())
}