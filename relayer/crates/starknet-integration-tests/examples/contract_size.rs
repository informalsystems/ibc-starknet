use cairo_lang_starknet_classes::casm_contract_class::CasmContractClass;
use cairo_lang_starknet_classes::contract_class::ContractClass;
use starknet::core::types::contract::SierraClass;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_path = std::env::args().nth(1).unwrap();

    let contract_filename = std::path::Path::new(&contract_path)
        .file_name()
        .ok_or("invalid path")?
        .to_string_lossy();

    let contract_str: String = std::fs::read_to_string(&contract_path)?;

    let contract_class: SierraClass = serde_json::from_str(&contract_str)?;

    let sierra_class_json = serde_json::to_string(&contract_class)?;

    let contract_class: ContractClass = serde_json::from_str(&sierra_class_json)?;

    let sierra_size = contract_class.sierra_program.len();

    let casm_contract = CasmContractClass::from_contract_class(contract_class, false, 180_000)?;

    let casm_size = casm_contract.bytecode.len();

    println!("{contract_filename}: sierra {sierra_size} felts, casm {casm_size} felts");

    Ok(())
}
