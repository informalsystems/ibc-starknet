use cairo_lang_starknet_classes::casm_contract_class::CasmContractClass;
use cairo_lang_starknet_classes::contract_class::ContractClass;
use starknet::core::types::contract::SierraClass;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_path = std::env::args().nth(1).unwrap();

    let contract_str: String = std::fs::read_to_string(&contract_path)?;

    println!("Contract: {contract_path}");

    let contract_class: SierraClass = serde_json::from_str(&contract_str)?;

    let sierra_class_json = serde_json::to_string(&contract_class)?;

    let contract_class: ContractClass = serde_json::from_str(&sierra_class_json)?;

    println!("  Sierra size: {}", contract_class.sierra_program.len());

    let casm_contract = CasmContractClass::from_contract_class(contract_class, false, 180_000)?;

    println!("  Casm size: {}", casm_contract.bytecode.len());

    Ok(())
}
