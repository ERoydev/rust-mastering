// Dynamic Enum Dispatch Pattern (Strategy via Trait Objects)

1. Enum         --> Describes types of assets (or strategies)
2. Trait        --> Common behavior across all strategies
3. Implementors --> Concrete logic for each enum variant
4. Resolver     --> Maps enum variant to a strategy
5. Usage        --> Call dynamic behavior via trait object

// 1. Lets say i have an enum and i want dynamic handler in my API endpoint to handle different types
#[derive(Deserialize, Debug)]
pub enum AssetTypes {
    ETH,
    ERC20,
    // Add more assets here
}

// 2. I define a trait used to apply this polymorphism that i want (Strategy Interface)
pub trait AssetHandler {
    fn handle(&self) -> String;
}

// 3. Implement trait for each strategy
pub struct EthHandler;
pub struct Erc20Handler;

impl AssetHandler for EthHandler {
    fn handle(&self) -> String {
        "Handling ETH asset".into()
    }
}

impl AssetHandler for Erc20Handler {
    fn handle(&self) -> String {
        "Handling ERC20 asset".into()
    }
}

// 4. Resolver function (Factory/Registry)
use std::sync::Arc;
pub fn resolve_handler(asset_type: &AssetTypes) -> Arc<dyn AssetHandler + Send + Sync> {
    match asset_type { // Have in mind that this can be automated with macro_rules !
        AssetTypes::ETH => Arc::new(EthHandler),
        AssetTypes::ERC20 => Arc::new(Erc20Handler),
    }
}
// For a fully dynamic plugin style, you can use a HashMap<AssetTypes, Arc<dyn AssetHandler>>, but the above keeps it simple and clean.


// 5. Use in Business Logic
pub async fn generate_proof(
    Json(payload): Json<GenerateProof>,
) -> (StatusCode, Json<Proof>) {
    let handler = resolve_handler(&payload.asset_type);
    let result = handler.handle(); // calls the correct logic

    let proof = Proof {
        mocked: result,
    };

    (StatusCode::CREATED, Json(proof))
}
