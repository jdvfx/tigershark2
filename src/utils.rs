use crate::errors::ErrOutput;
use crate::parse_args::Asset;

// CRUD functions

pub fn create(collection: String, asset: Asset) -> ErrOutput {
    // > required:
    // asset_name, location, source

    println!("create");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "asset created".to_owned(),
    }
}
pub fn update(collection: String, asset: Asset) -> ErrOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    println!("update asset");

    // get latest version and increment
    // create new Version struct and push to Vec > add to collection

    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "asset updated".to_owned(),
    }
}
pub fn get_source(collection: String, asset: Asset) -> ErrOutput {
    // > required:
    // datapath

    println!("get source");

    // parse datapath and extract asset_name, location, version ?
    // quiery and return source.

    println!("collection: {collection}");
    println!("Asset: {:?}", asset);

    ErrOutput {
        status: 0,
        output: "source file: xxxx".to_owned(),
    }
}

pub fn delete(collection: String, asset: Asset) -> ErrOutput {
    // > required:
    // asset_name, location, source, version
    // OR
    // asset_id, version

    println!("mark asset for deletion");

    // find asset and update status to "purge"
    // status should be an Enum: online/purge/deleted

    println!("collection: {collection}");
    println!("Asset: {:?}", asset);
    ErrOutput {
        status: 0,
        output: "asset marked for deletion".to_owned(),
    }
}

pub fn get_latest(collection: String, asset: Asset) -> ErrOutput {
    // > required:
    // asset_name, location, source
    // OR
    // asset_id

    println!("get latest verions");
    println!("collection: {collection}");
    println!("Asset: {:?}", asset);
    ErrOutput {
        status: 0,
        output: "latest version is xxx".to_owned(),
    }
}
