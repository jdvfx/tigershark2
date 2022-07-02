use crate::errors::ErrOutput;
use crate::parse_args::Asset;

pub fn create(collection: String, asset: Asset) -> ErrOutput {
    println!("create");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "asset created".to_owned(),
    }
}
pub fn update(collection: String, asset: Asset) -> ErrOutput {
    println!("update asset");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "asset updated".to_owned(),
    }
}
pub fn get_source(collection: String, asset: Asset) -> ErrOutput {
    println!("get source");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "source file: xxxx".to_owned(),
    }
}

pub fn delete(collection: String, asset: Asset) -> ErrOutput {
    println!("mark asset for deletion");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);
    ErrOutput {
        status: 0,
        output: "asset marked for deletion".to_owned(),
    }
}

pub fn get_latest(collection: String, asset: Asset) -> ErrOutput {
    println!("get latest verions");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);
    ErrOutput {
        status: 0,
        output: "latest version is xxx".to_owned(),
    }
}
